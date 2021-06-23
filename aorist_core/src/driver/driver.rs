#![allow(dead_code)]
use crate::code::CodeBlockWithDefaultConstructor;
use crate::concept::{Concept, ConceptAncestry, AoristRef};
use crate::universe::Universe;
use crate::constraint::SatisfiableOuterConstraint;
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::dialect::Dialect;
use crate::endpoints::EndpointConfig;
use crate::flow::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use aorist_ast::{AncestorRecord, SimpleIdentifier, AST};
use crate::constraint::TConstraintEnum;
use crate::constraint::{OuterConstraint, TBuilder};
use aorist_primitives::{Ancestry, TConceptEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use tracing::{debug, trace, level_enabled, Level};
use uuid::Uuid;
use aorist_primitives::TAoristObject;

pub type ConstraintsBlockMap<'a, C> = LinkedHashMap<
    String,
    (
        LinkedHashSet<String>,
        LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, C>>>>,
    ),
>;

pub trait Driver<'a, B, D, U>
where
    B: TBuilder<'a>,
    D: FlowBuilderBase,
    D:
        FlowBuilderMaterialize<
            BuilderInputType = <Self::CB as ConstraintBlock<
                'a,
                <D as FlowBuilderBase>::T,
                B::OuterType,
            >>::BuilderInputType,
        >,
    <D as FlowBuilderBase>::T: 'a,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept: TConceptEnum<TUniverse=U>
{
    type CB: ConstraintBlock<'a, <D as FlowBuilderBase>::T, B::OuterType>;

    fn get_relevant_builders(
        topline_constraint_names: &LinkedHashSet<String>,
    ) -> Vec<B> {
        let mut builders = B::builders()
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
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, B::OuterType>>;

    fn find_satisfiable_constraint_block(
        &self,
        unsatisfied_constraints: &mut ConstraintsBlockMap<'a, B::OuterType>,
    ) -> Option<(
        LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, B::OuterType>>>>,
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
        block: &LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, B::OuterType>>>>,
        constraint_name: String,
    ) -> Option<AST> {
        match block.len() == 1 {
            true => None,
            false => Some(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                format!("tasks_{}", constraint_name).to_string(),
            ))),
        }
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, String)) -> Arc<RwLock<B::OuterType>>;
    fn get_preferences(&self) -> Vec<Dialect>;
    fn get_ancestry(&self) -> Arc<<B::OuterType as OuterConstraint<'a>>::TAncestry>;
    fn process_constraint_with_program(
        &mut self,
        constraint: RwLockReadGuard<'_, B::OuterType>,
        uuid: (Uuid, String),
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        state: Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
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
        state: Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, B::OuterType>,
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
        state: Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
    );
    fn process_constraint_block(
        &mut self,
        block: &mut LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, B::OuterType>>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        constraint_name: String,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, B::OuterType>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Result<(
        Vec<<Self::CB as ConstraintBlock<'a, <D as FlowBuilderBase>::T, B::OuterType>>::C>,
        Option<AST>,
    )> {
        let tasks_dict = Self::init_tasks_dict(block, constraint_name.clone());
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, ParameterTuple)>> =
            HashMap::new();
        let mut blocks = Vec::new();
        let mut by_dialect: HashMap<Option<Dialect>, Vec<Arc<RwLock<ConstraintState<'a, B::OuterType>>>>> =
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
            let block =
                <Self::CB as ConstraintBlock<'a, <D as FlowBuilderBase>::T, B::OuterType>>::C::new(
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
    fn get_endpoints(&self) -> AoristRef<EndpointConfig>;
    fn get_dependencies(&self) -> Vec<String>;
    fn run(&mut self) -> Result<(String, Vec<String>)> {
        self.satisfy_constraints()?;
        let etl = D::new();
        let endpoints = self.get_endpoints().clone();
        let statements_and_preambles = self
            .get_blocks()
            .iter()
            .map(|x| x.get_statements(endpoints.clone()))
            .collect::<Vec<_>>();

        Ok((
            etl.materialize(statements_and_preambles)?,
            self.get_dependencies(),
        ))
    }
    fn get_blocks(&self) -> &Vec<Self::CB>;
    fn _new(
        concepts: Arc<RwLock<HashMap<(Uuid, String), AoristRef<Concept>>>>,
        constraints: LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        ancestry: Arc<ConceptAncestry>,
        endpoints: AoristRef<EndpointConfig>,
        ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<String>,
    ) -> Self;

    fn generate_constraint_states_map(
        constraints: &LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        concepts: Arc<
            RwLock<
                HashMap<
                    (Uuid, String),
                    <<B::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
    ) -> Result<LinkedHashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a, B::OuterType>>>>> {
        let mut states_map = LinkedHashMap::new();
        for (k, rw) in constraints {
            states_map.insert(
                k.clone(),
                Arc::new(RwLock::new(ConstraintState::new(
                    rw.clone(),
                    concepts.clone(),
                    ancestors,
                )?)),
            );
        }
        Ok(states_map)
    }
    fn remove_redundant_dependencies(
        raw_unsatisfied_constraints: &mut LinkedHashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
        >,
    ) {
        /* Remove redundant dependencies */
        // constraint key => constraint key dependency on it
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                for dep in v.read().unwrap().unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(k.clone());
                }
            }

            let tips = raw_unsatisfied_constraints
                .iter()
                .filter(|(k, _v)| !reverse_dependencies.contains_key(k));
            for tip in tips {
                let mut visits: HashMap<(Uuid, String), (Uuid, String)> = HashMap::new();
                let mut queue: VecDeque<((Uuid, String), Arc<RwLock<_>>)> = VecDeque::new();
                queue.push_back((tip.0.clone(), tip.1.clone()));
                while queue.len() > 0 {
                    let (key, elem) = queue.pop_front().unwrap();
                    let new_deps = elem.read().unwrap().unsatisfied_dependencies.clone();
                    for dep in new_deps {
                        let dep_constraint = raw_unsatisfied_constraints.get(&dep).unwrap().clone();
                        // we have already visited this dependency
                        if visits.contains_key(&dep) {
                            // this is the key of the constraint through which we have already visited
                            let prev = visits.remove(&dep).unwrap();
                            if prev != key {
                                let prev_constraint =
                                    raw_unsatisfied_constraints.get(&prev).unwrap();
                                let mut write = prev_constraint.write().unwrap();
                                if write.get_name() != elem.read().unwrap().get_name() {
                                    assert!(write.unsatisfied_dependencies.remove(&dep));
                                    changes_made = true;
                                }
                            }
                        }
                        visits.insert(dep.clone(), key.clone());
                        queue.push_back((dep, dep_constraint));
                    }
                }
            }
        }
    }
    fn remove_superfluous_dummy_tasks(
        raw_unsatisfied_constraints: &mut LinkedHashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
        >,
    ) -> Result<()> {
        /* Remove superfluous dummy tasks */
        loop {
            let mut superfluous = Vec::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                let x = v.read().unwrap();
                if x.unsatisfied_dependencies.len() == 1 && !(x.requires_program()?) {
                    superfluous.push(k.clone());
                }
            }
            if let Some(elem) = superfluous.into_iter().next() {
                let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                    LinkedHashMap::new();
                for (k, v) in raw_unsatisfied_constraints.iter() {
                    for dep in v.read().unwrap().unsatisfied_dependencies.iter() {
                        reverse_dependencies
                            .entry(dep.clone())
                            .or_insert(Vec::new())
                            .push(k.clone());
                    }
                }
                let arc = raw_unsatisfied_constraints.remove(&elem).unwrap();
                let dep = arc
                    .read()
                    .unwrap()
                    .unsatisfied_dependencies
                    .iter()
                    .next()
                    .unwrap()
                    .clone();

                if let Some(rev_deps) = reverse_dependencies.get(&elem) {
                    for rev in rev_deps.iter() {
                        let mut write = raw_unsatisfied_constraints
                            .get(rev)
                            .unwrap()
                            .write()
                            .unwrap();
                        assert!(write.unsatisfied_dependencies.remove(&elem));
                        write.unsatisfied_dependencies.insert(dep.clone());
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }
    fn remove_dangling_dummy_tasks(
        raw_unsatisfied_constraints: &mut LinkedHashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
        >,
    ) -> Result<()> {
        /* Remove dangling dummy tasks */
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let mut dangling = Vec::new();
            let mut reverse_dependencies: LinkedHashMap<(Uuid, String), Vec<(Uuid, String)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                let x = v.read().unwrap();
                if x.unsatisfied_dependencies.len() == 0 && !(x.requires_program()?) {
                    dangling.push(k.clone());
                }

                trace!("Reverse dependencies for {:?}", k);
                for dep in x.unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(Vec::new())
                        .push(k.clone());
                    trace!("- {:?}", dep);
                }
            }
            for k in dangling {
                assert!(raw_unsatisfied_constraints.remove(&k).is_some());
                trace!("Dangling: {:?}", k);
                if let Some(v) = reverse_dependencies.get(&k) {
                    for rev in v {
                        assert!(raw_unsatisfied_constraints
                            .get(rev)
                            .unwrap()
                            .write()
                            .unwrap()
                            .unsatisfied_dependencies
                            .remove(&k));
                    }
                }
                changes_made = true;
            }
        }
        Ok(())
    }
    fn get_unsatisfied_constraints(
        constraints: &LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        concepts: Arc<
            RwLock<
                HashMap<
                    (Uuid, String),
                    <<B::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
        _topline_constraint_names: LinkedHashSet<String>,
    ) -> Result<ConstraintsBlockMap<'a, B::OuterType>> {
        let mut raw_unsatisfied_constraints: LinkedHashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a, B::OuterType>>>,
        > = Self::generate_constraint_states_map(constraints, concepts, ancestors)?;
        Self::remove_redundant_dependencies(&mut raw_unsatisfied_constraints);
        Self::remove_superfluous_dummy_tasks(&mut raw_unsatisfied_constraints)?;
        Self::remove_dangling_dummy_tasks(&mut raw_unsatisfied_constraints)?;

        let mut unsatisfied_constraints: LinkedHashMap<_, _> = <<B::OuterType as OuterConstraint<'a>>::TEnum as TConstraintEnum<'a>>::get_required_constraint_names()
            .into_iter()
            .map(|(k, v)| (k, (v.into_iter().collect(), LinkedHashMap::new())))
            .collect();

        for ((uuid, root_type), rw) in raw_unsatisfied_constraints.into_iter() {
            let constraint_name = rw.read().unwrap().get_name();
            unsatisfied_constraints
                .get_mut(&constraint_name)
                .unwrap()
                .1
                .insert((uuid, root_type), rw);
        }

        Ok(unsatisfied_constraints)
    }
    fn get_concept_map_by_object_type(
        concept_map: HashMap<(Uuid, String), AoristRef<Concept>>,
    ) -> HashMap<String, Vec<AoristRef<Concept>>> {
        let mut by_object_type: HashMap<String, Vec<AoristRef<Concept>>> = HashMap::new();
        debug!("Found the following concepts:");
        for ((uuid, object_type), concept) in concept_map {
            debug!("- {}: {}", &object_type, &uuid);
            by_object_type
                .entry(object_type)
                .or_insert(Vec::new())
                .push(concept.clone());
        }
        by_object_type
    }
    fn compute_all_ancestors(
        universe: AoristRef<Concept>,
        concept_map: &HashMap<(Uuid, String), AoristRef<Concept>>,
    ) -> HashMap<(Uuid, String), Vec<AncestorRecord>> {
        let mut ancestors: HashMap<(Uuid, String), Vec<AncestorRecord>> = HashMap::new();
        let mut frontier: Vec<AncestorRecord> = Vec::new();
        frontier.push(AncestorRecord::new(
            universe.get_uuid(),
            universe.get_type(),
            universe.get_tag(),
            universe.get_index_as_child(),
        ));
        ancestors.insert(
            (universe.get_uuid(), universe.get_type()),
            vec![AncestorRecord::new(
                universe.get_uuid(),
                universe.get_type(),
                None,
                0,
            )],
        );
        while frontier.len() > 0 {
            let mut new_frontier: Vec<AncestorRecord> = Vec::new();
            for child in frontier.drain(0..) {
                let key = child.get_key();
                let concept = concept_map.get(&key).unwrap();
                let child_ancestors = ancestors.get(&key).unwrap().clone();
                for grandchild in concept.get_child_concepts() {
                    let t = AncestorRecord::new(
                        grandchild.get_uuid(),
                        grandchild.get_type(),
                        grandchild.get_tag(),
                        grandchild.get_index_as_child(),
                    );
                    new_frontier.push(t.clone());
                    let mut grandchild_ancestors = child_ancestors.clone();
                    grandchild_ancestors.push(t);
                    ancestors.insert(
                        (grandchild.get_uuid(), grandchild.get_type()),
                        grandchild_ancestors,
                    );
                }
            }
            frontier = new_frontier;
        }
        ancestors
    }
    fn new(
        universe: AoristRef<Universe>,
        topline_constraint_names: LinkedHashSet<String>
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let sorted_builders = Self::get_relevant_builders(&topline_constraint_names);
        let mut concept_map: HashMap<(Uuid, String), AoristRef<Concept>> = HashMap::new();
        let concept = AoristRef(Arc::new(RwLock::new(Concept::Universe((universe.clone(), 0, None)))));
        concept.populate_child_concept_map(&mut concept_map);
        let by_object_type = Self::get_concept_map_by_object_type(concept_map.clone());

        let ancestors = Self::compute_all_ancestors(concept, &concept_map);
        let mut visited_constraint_names: LinkedHashSet<String> = LinkedHashSet::new();
        // constraint_name => root_id => constraint_object
        let mut generated_constraints: LinkedHashMap<
            String,
            LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        > = LinkedHashMap::new();

        let concepts = Arc::new(RwLock::new(concept_map));
        let ancestry: ConceptAncestry = ConceptAncestry {
            parents: concepts.clone(),
        };
        let family_trees = Self::generate_family_trees(&ancestors);

        for builder in &sorted_builders {
            Self::attach_constraints(
                builder,
                &by_object_type,
                &family_trees,
                &ancestry,
                &mut generated_constraints,
                &mut visited_constraint_names,
            )?;
        }

        let mut constraints = LinkedHashMap::new();
        for (_k, v) in generated_constraints {
            for ((_root_id, root_type), rw) in v.into_iter() {
                constraints.insert(
                    (rw.read().unwrap().get_uuid()?.clone(), root_type),
                    rw.clone(),
                );
            }
        }
        Ok(Self::_new(
            concepts,
            constraints,
            Arc::new(ancestry),
            universe.0.read().unwrap().endpoints.clone(),
            ancestors,
            topline_constraint_names,
        ))
    }
    fn generate_family_trees(
        ancestors: &HashMap<(Uuid, String), Vec<AncestorRecord>>,
    ) -> HashMap<(Uuid, String), HashMap<String, HashSet<Uuid>>> {
        let mut family_trees: HashMap<(Uuid, String), HashMap<String, HashSet<Uuid>>> =
            HashMap::new();
        for (key, ancestor_v) in ancestors.iter() {
            for record in ancestor_v {
                family_trees
                    .entry(key.clone())
                    .or_insert(HashMap::new())
                    .entry(record.object_type.clone())
                    .or_insert(HashSet::new())
                    .insert(record.uuid);
            }
            for record in ancestor_v {
                let (uuid, object_type) = key;
                let ancestor_key = record.get_key();
                family_trees
                    .entry(ancestor_key)
                    .or_insert(HashMap::new())
                    .entry(object_type.clone())
                    .or_insert(HashSet::new())
                    .insert(uuid.clone());
            }
            let (uuid, object_type) = key;
            family_trees
                .entry(key.clone())
                .or_insert(HashMap::new())
                .entry(object_type.clone())
                .or_insert(HashSet::new())
                .insert(uuid.clone());
        }
        family_trees
    }
    fn attach_constraints(
        builder: &B,
        by_object_type: &HashMap<String, Vec<AoristRef<Concept>>>,
        family_trees: &HashMap<(Uuid, String), HashMap<String, HashSet<Uuid>>>,
        ancestry: &ConceptAncestry,
        generated_constraints: &mut LinkedHashMap<
            String,
            LinkedHashMap<(Uuid, String), Arc<RwLock<B::OuterType>>>,
        >,
        visited_constraint_names: &mut LinkedHashSet<String>,
    ) -> Result<()> {
        let root_object_type = builder.get_root_type_name()?;
        let constraint_name = builder.get_constraint_name();

        if let Some(root_concepts) = by_object_type.get(&root_object_type) {
            debug!(
                "Attaching constraint {} to {} objects of type {}.",
                constraint_name,
                root_concepts.len(),
                root_object_type
            );

            for root in root_concepts {
                let root_key = (root.get_uuid(), root.get_type());
                let family_tree = family_trees.get(&root_key).unwrap();
                let raw_potential_child_constraints = builder
                    .get_required_constraint_names()
                    .into_iter()
                    .map(|req| (req.clone(), generated_constraints.get(&req)))
                    .filter(|(_req, x)| x.is_some())
                    .map(|(req, x)| (req, x.unwrap()))
                    .collect::<Vec<_>>();
                if level_enabled!(Level::DEBUG) {
                    debug!(
                        "Creating constraint {:?} on root {:?} with potential child constraints:",
                        builder.get_constraint_name(),
                        &root_key
                    );
                    for (required_constraint_name, map) in raw_potential_child_constraints.iter() {
                        debug!(" - for {}:", required_constraint_name);
                        for (key, v) in map.iter() {
                            let downstream = v.read().unwrap();
                            debug!(
                                " -- {:?}: {:?}",
                                key,
                                (downstream.get_uuid()?, downstream.get_name())
                            );
                        }
                    }
                }
                let other_required_concept_uuids = builder
                    .get_required(root.clone(), &ancestry)
                    .into_iter()
                    .collect::<HashSet<_>>();
                let potential_child_constraints = raw_potential_child_constraints
                    .into_iter()
                    .map(|(_req, x)| {
                        x.iter()
                            .filter(|((potential_root_id, potential_root_type), _constraint)| {
                                (match family_tree.get(potential_root_type) {
                                    None => false,
                                    Some(set) => set.contains(potential_root_id),
                                } || other_required_concept_uuids.contains(potential_root_id))
                            })
                            .map(|(_, constraint)| constraint.clone())
                    })
                    .flatten()
                    .collect::<Vec<Arc<RwLock<B::OuterType>>>>();
                if builder.should_add(root.clone(), &ancestry) {
                    if level_enabled!(Level::DEBUG) {
                        debug!("After filtering:",);
                        for downstream_rw in potential_child_constraints.iter() {
                            let downstream = downstream_rw.read().unwrap();
                            debug!(" --  {:?}", (downstream.get_uuid()?, downstream.get_name()));
                        }
                    }
                    let constraint =
                        builder.build_constraint(root.get_uuid(), potential_child_constraints)?;
                    let gen_for_constraint = generated_constraints
                        .entry(constraint_name.clone())
                        .or_insert(LinkedHashMap::new());
                    assert!(!gen_for_constraint.contains_key(&root_key));
                    if level_enabled!(Level::DEBUG) {
                        debug!(
                            "Added constraint {:?} on root {:?} with the following dependencies:",
                            (constraint.get_uuid()?, constraint.get_name()),
                            &root_key
                        );
                        for downstream_rw in constraint.get_downstream_constraints()? {
                            let downstream = downstream_rw.read().unwrap();
                            debug!(" --  {:?}", (downstream.get_uuid()?, downstream.get_name()));
                        }
                    }
                    gen_for_constraint.insert(root_key, Arc::new(RwLock::new(constraint)));
                }
            }
        } else {
            debug!(
                "Found no concepts of type {} for {}",
                root_object_type, constraint_name,
            );
        }
        for req in builder.get_required_constraint_names() {
            assert!(visited_constraint_names.contains(&req));
        }
        visited_constraint_names.insert(constraint_name.clone());
        Ok(())
    }
}
