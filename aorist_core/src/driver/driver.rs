#![allow(dead_code)]
use crate::code::CodeBlockWithDefaultConstructor;
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python, R};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::parameter_tuple::ParameterTuple;
#[cfg(feature = "python")]
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
#[cfg(feature = "r")]
use crate::r::{RBasedConstraintBlock, RFlowBuilderInput, RImport, RPreamble};
use crate::universe::Universe;
use anyhow::Result;
use aorist_ast::{AncestorRecord, SimpleIdentifier, AST};
use aorist_primitives::{OuterConstraint, TConstraint, TBuilder};
use aorist_primitives::{TAoristObject, TConceptEnum, TConstraintEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

type ConstraintsBlockMap<'a, 'b, C> = LinkedHashMap<
    String,
    (
        LinkedHashSet<String>,
        LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
    ),
>;

pub trait Driver<'a, 'b, D, C>
where
    D: FlowBuilderBase,
    D:
        FlowBuilderMaterialize<
            BuilderInputType = <Self::CB as ConstraintBlock<
                'a,
                'b,
                <D as FlowBuilderBase>::T,
                C,
            >>::BuilderInputType,
        >,
    <D as FlowBuilderBase>::T: 'a,
    Self::CB: 'a,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a: 'b,
{
    type CB: ConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>;

    fn get_relevant_builders(
        topline_constraint_names: &LinkedHashSet<String>,
    ) -> Vec<<<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<'a, 'b>>::BuilderT> {
        let mut builders =
            <<C as OuterConstraint<'a, 'b>>::TEnum as TConstraintEnum<'a, 'b>>::builders()
                .into_iter()
                .map(|x| (x.get_constraint_name(), x))
                .collect::<LinkedHashMap<String, _>>();
        let mut builder_q = topline_constraint_names
            .clone()
            .into_iter()
            .map(|x| {
                (
                    x.clone(),
                    builders
                        .remove(&x)
                        .expect(format!("Missing constraint named {}", x).as_str()),
                )
            })
            .collect::<VecDeque<_>>();
        let mut relevant_builders = LinkedHashMap::new();
        let mut visited = HashSet::new();
        let mut g: LinkedHashMap<String, LinkedHashSet<String>> = LinkedHashMap::new();
        let mut rev: HashMap<String, Vec<String>> = HashMap::new();

        while builder_q.len() > 0 {
            let (key, builder) = builder_q.pop_front().unwrap();
            let edges = g.entry(key.clone()).or_insert(LinkedHashSet::new());
            debug!("Constraint {} requires:", key);
            for req in builder.get_required_constraint_names() {
                debug!("  - {}", req);
                if !visited.contains(&req) {
                    let another = builders.remove(&req).unwrap();
                    builder_q.push_back((req.clone(), another));
                    visited.insert(req.clone());
                }
                edges.insert(req.clone());
                let rev_edges = rev.entry(req.clone()).or_insert(Vec::new());
                rev_edges.push(key.clone());
            }
            relevant_builders.insert(key.clone(), builder);
        }
        let mut sorted_builders = Vec::new();
        while g.len() > 0 {
            let leaf = g
                .iter()
                .filter(|(_, v)| v.len() == 0)
                .map(|(k, _)| k)
                .next()
                .unwrap()
                .clone();

            let builder = relevant_builders.remove(&leaf).unwrap();
            if let Some(parents) = rev.remove(&leaf) {
                for parent in parents {
                    g.get_mut(&parent).unwrap().remove(&leaf);
                }
            }
            sorted_builders.push(builder);
            g.remove(&leaf);
        }

        sorted_builders
    }
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, 'b, C>>;

    fn find_satisfiable_constraint_block(
        &self,
        unsatisfied_constraints: &mut ConstraintsBlockMap<'a, 'b, C>,
    ) -> Option<(
        LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
        String,
    )> {
        let constraint_block_name = unsatisfied_constraints
            .iter()
            .filter(|(_, v)| v.0.len() == 0)
            .map(|(k, _)| k.clone())
            .next();
        match constraint_block_name {
            Some(name) => {
                let (_dependency_names, constraints) =
                    unsatisfied_constraints.remove(&name).unwrap();
                for (_, (v, _)) in unsatisfied_constraints.iter_mut() {
                    v.remove(&name);
                }
                Some((constraints, name))
            }
            None => None,
        }
    }
    fn init_tasks_dict(
        block: &LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
        constraint_name: String,
    ) -> Option<AST> {
        match block.len() == 1 {
            true => None,
            false => Some(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                format!("tasks_{}", constraint_name).to_string(),
            ))),
        }
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<C>>;
    fn get_preferences(&self) -> Vec<Dialect>;
    fn get_ancestry(&self) -> Arc<<C as OuterConstraint<'a, 'b>>::TAncestry>;
    fn process_constraint_with_program(
        &mut self,
        constraint: RwLockReadGuard<'_, C>,
        uuid: (Uuid, String),
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        state: Arc<RwLock<ConstraintState<'a, 'b, C>>>,
    ) {
        let name = constraint.get_name().clone();
        drop(constraint);
        // TODO: turn into a reference to a field on self
        let preferences = self.get_preferences();
        let mut write = state.write().unwrap();
        // TODO: remove dummy hash map
        write.satisfy(&preferences, self.get_ancestry());
        drop(write);

        // TODO: preambles and calls are superflous
        let key = state.read().unwrap().key.as_ref().unwrap().clone();
        calls
            .entry((
                state.read().unwrap().get_call().unwrap(),
                name,
                uuid.1.clone(),
            ))
            .or_insert(Vec::new())
            .push((key, state.read().unwrap().get_params().unwrap()));
    }
    fn process_constraint_state(
        &mut self,
        uuid: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, 'b, C>>>,
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, 'b, C>,
    ) -> Result<()> {
        let read = state.read().unwrap();
        assert!(!read.satisfied);
        assert_eq!(read.unsatisfied_dependencies.len(), 0);
        drop(read);

        let rw = self.get_constraint_rwlock(&uuid);
        let constraint = rw.read().unwrap();

        if constraint.requires_program()? {
            self.process_constraint_with_program(constraint, uuid.clone(), calls, state.clone());
        }

        if let Some(v) = reverse_dependencies.get(&uuid) {
            for (dependency_name, dependency_uuid, dependency_root_type) in v {
                let rw = unsatisfied_constraints
                    .get(dependency_name)
                    .unwrap()
                    .1
                    .get(&(*dependency_uuid, dependency_root_type.clone()))
                    .unwrap();
                let mut write = rw.write().unwrap();
                write.satisfied_dependencies.push(state.clone());
                assert!(write.unsatisfied_dependencies.remove(&uuid));
                drop(write);
            }
        }

        let mut write = state.write().unwrap();
        write.satisfied = true;
        drop(write);
        Ok(())
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, 'b, C>>>,
    );
    fn process_constraint_block(
        &mut self,
        block: &mut LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, 'b, C>>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        constraint_name: String,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, 'b, C>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<(
        Vec<<Self::CB as ConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>>::C>,
        Option<AST>,
    )> {
        let tasks_dict = Self::init_tasks_dict(block, constraint_name.clone());
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, ParameterTuple)>> =
            HashMap::new();
        let mut blocks = Vec::new();
        let mut by_dialect: HashMap<Option<Dialect>, Vec<Arc<RwLock<ConstraintState<'a, 'b, C>>>>> =
            HashMap::new();
        for (id, state) in block.clone() {
            self.process_constraint_state(
                id.clone(),
                state.clone(),
                &mut calls,
                reverse_dependencies,
                unsatisfied_constraints,
            )?;
            self.mark_constraint_state_as_satisfied(id, state.clone());
            by_dialect
                .entry(state.read().unwrap().get_dialect())
                .or_insert(Vec::new())
                .push(state.clone());
        }
        for (_dialect, satisfied) in by_dialect.into_iter() {
            let block = <Self::CB as ConstraintBlock<'a, 'b, <D as FlowBuilderBase>::T, C>>::C::new(
                satisfied,
                constraint_name.clone(),
                tasks_dict.clone(),
                identifiers,
            )?;
            blocks.push(block);
        }

        Ok((blocks, tasks_dict))
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &String,
    ) -> (Option<String>, Option<String>);
    fn add_block(&mut self, constraint_block: Self::CB);
    fn satisfy_constraints(&mut self) -> Result<()> {
        let mut unsatisfied_constraints = self.init_unsatisfied_constraints()?;
        let mut reverse_dependencies: HashMap<(Uuid, String), HashSet<(String, Uuid, String)>> =
            HashMap::new();
        for (name, (_, constraints)) in &unsatisfied_constraints {
            for ((uuid, root_type), state) in constraints {
                for (dependency_uuid, dependency_root_type) in
                    &state.read().unwrap().unsatisfied_dependencies
                {
                    reverse_dependencies
                        .entry((*dependency_uuid, dependency_root_type.clone()))
                        .or_insert(HashSet::new())
                        .insert((name.clone(), *uuid, root_type.clone()));
                }
            }
        }

        let mut existing_names = HashSet::new();
        let mut identifiers = HashMap::new();
        // find at least one satisfiable constraint
        loop {
            let mut satisfiable =
                self.find_satisfiable_constraint_block(&mut unsatisfied_constraints);
            if let Some((ref mut block, ref constraint_name)) = satisfiable {
                ConstraintState::shorten_task_names(block, &mut existing_names);
                let snake_case_name = to_snake_case(constraint_name);
                if block.len() > 0 {
                    let (members, tasks_dict) = self.process_constraint_block(
                        &mut block.clone(),
                        &reverse_dependencies,
                        snake_case_name.clone(),
                        &unsatisfied_constraints,
                        &identifiers,
                    )?;

                    let (title, body) = self.get_constraint_explanation(constraint_name);
                    let constraint_block =
                        Self::CB::new(snake_case_name, title, body, members, tasks_dict);
                    for (key, val) in constraint_block.get_identifiers() {
                        identifiers.insert(key, val);
                    }
                    self.add_block(constraint_block);
                }
            } else {
                assert_eq!(unsatisfied_constraints.len(), 0);
                return Ok(());
            }
        }
    }
    fn get_endpoints(&'b self) -> &'b EndpointConfig;
    fn get_dependencies(&self) -> Vec<String>;
    fn run(&'b mut self) -> Result<(String, Vec<String>)> {
        self.satisfy_constraints()?;
        let etl = D::new();
        let endpoints = self.get_endpoints().clone();
        let statements_and_preambles = self
            .get_blocks()
            .iter()
            .map(|x| x.get_statements(&endpoints))
            .collect::<Vec<_>>();

        Ok((
            etl.materialize(statements_and_preambles)?,
            self.get_dependencies(),
        ))
    }
    fn get_blocks(&'b self) -> &'b Vec<Self::CB>;
}
