#![allow(dead_code)]
use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_state::ConstraintState;
use crate::dialect::{Bash, Dialect, Presto, Python};
use crate::driver::{ConstraintsBlockMap, Driver};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderBase, FlowBuilderMaterialize};
use crate::python::{
    PythonBasedConstraintBlock, PythonFlowBuilderInput, PythonImport, PythonPreamble,
};
use anyhow::Result;
use aorist_ast::AncestorRecord;
use aorist_primitives::OuterConstraint;
use aorist_primitives::TConstraintEnum;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct PythonBasedDriver<'a, D, C>
where
    D: FlowBuilderBase,
    <D as FlowBuilderBase>::T:
        ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    C: OuterConstraint<'a, TAncestry = ConceptAncestry>
        + SatisfiableOuterConstraint<'a>
{
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept>>>,
    constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, C>>>>,
    blocks: Vec<PythonBasedConstraintBlock<'a, D::T, C>>,
    ancestry: Arc<ConceptAncestry>,
    dag_type: PhantomData<D>,
    endpoints: EndpointConfig,
    constraint_explanations: HashMap<String, (Option<String>, Option<String>)>,
    ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
    topline_constraint_names: LinkedHashSet<String>,
}

impl<'a, D, C> Driver<'a, D, C> for PythonBasedDriver<'a, D, C>
where
    D: FlowBuilderBase,
    D: FlowBuilderMaterialize<BuilderInputType = PythonFlowBuilderInput>,
    <D as FlowBuilderBase>::T:
        ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble> + 'a,
    C: OuterConstraint<'a, TAncestry = ConceptAncestry>
        + SatisfiableOuterConstraint<'a>
{
    type CB = PythonBasedConstraintBlock<'a, <D as FlowBuilderBase>::T, C>;

    fn get_preferences(&self) -> Vec<Dialect> {
        vec![
            Dialect::Python(Python::new(vec![])),
            Dialect::Presto(Presto {}),
            Dialect::Bash(Bash {}),
        ]
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<C>> {
        self.constraints.get(uuid).unwrap().clone()
    }

    fn get_endpoints(&self) -> &EndpointConfig {
        &self.endpoints
    }

    fn get_ancestry(&self) -> Arc<ConceptAncestry> {
        self.ancestry.clone()
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a, C>>>,
    ) {
        self.satisfied_constraints.insert(id, state.clone());
    }
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, C>> {
        Self::get_unsatisfied_constraints(
            &self.constraints,
            self.concepts.clone(),
            &self.ancestors,
            self.topline_constraint_names.clone(),
        )
    }
    fn add_block(
        &mut self,
        constraint_block: PythonBasedConstraintBlock<'a, <D as FlowBuilderBase>::T, C>,
    ) {
        self.blocks.push(constraint_block);
    }
    fn get_constraint_explanation(
        &self,
        constraint_name: &String,
    ) -> (Option<String>, Option<String>) {
        self.constraint_explanations
            .get(constraint_name)
            .unwrap()
            .clone()
    }
    fn get_blocks(&self) -> &Vec<Self::CB> {
        &self.blocks
    }
    fn get_dependencies(&self) -> Vec<String> {
        self.satisfied_constraints
            .values()
            .map(|x| match x.read().unwrap().get_dialect() {
                Some(Dialect::Python(x)) => Some(x.get_pip_requirements()),
                _ => None,
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap().into_iter())
            .flatten()
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect()
    }
    fn _new(
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept>>>,
        constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<C>>>,
        ancestry: Arc<ConceptAncestry>,
        endpoints: EndpointConfig,
        ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<String>,
    ) -> Self {
        Self {
            concepts,
            constraints,
            satisfied_constraints: HashMap::new(),
            blocks: Vec::new(),
            ancestry,
            dag_type: PhantomData,
            endpoints,
            constraint_explanations: <<C as OuterConstraint<'a>>::TEnum as TConstraintEnum<
                'a,
            >>::get_explanations(),
            ancestors,
            topline_constraint_names,
        }
    }
}
