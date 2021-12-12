use aorist_primitives::AOption;
use abi_stable::std_types::ROption;

use crate::code::CodeBlock;
use crate::code::CodeBlockWithDefaultConstructor;
use crate::constraint::TConstraintEnum;
use crate::constraint::{OuterConstraint, TBuilder};
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::dialect::Dialect;
use crate::flow::{FlowBuilderBase, FlowBuilderMaterialize};
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
//use crate::task_name_shortener::TaskNameShortener;
use abi_stable::external_types::parking_lot::rw_lock::{RReadGuard, RRwLock};
use abi_stable::std_types::RArc;
use anyhow::Result;
use aorist_ast::{AncestorRecord, SimpleIdentifier, AST};
use aorist_primitives::{AString, AVec, TAoristObject};
use aorist_primitives::{Ancestry, AoristConcept, AoristUniverse, TConceptEnum};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, level_enabled, trace, Level};
use uuid::Uuid;

pub type ConstraintsBlockMap<'a, C, P> = LinkedHashMap<
    AString,
    (
        LinkedHashSet<AString>,
        LinkedHashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, C, P>>>>,
    ),
>;

pub trait Driver<'a, B, D, U, C, A, P>
where
    U: AoristConcept + AoristUniverse,
    B: TBuilder<'a, TEnum = C, TAncestry = A>,
    D: FlowBuilderBase<U>,
    D: FlowBuilderMaterialize<
        U,
        BuilderInputType = <Self::CB as ConstraintBlock<
            'a,
            <D as FlowBuilderBase<U>>::T,
            B::OuterType,
            U,
            P,
        >>::BuilderInputType,
    >,
    <D as FlowBuilderBase<U>>::T: 'a,
    A: Ancestry,
    C: TConceptEnum<TUniverse = U>,
    <B as TBuilder<'a>>::OuterType: OuterConstraint<'a, TAncestry = A>,
    <<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry: Ancestry<TConcept = C>,
    <<<B as TBuilder<'a>>::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept:
        TConceptEnum<TUniverse = U>,
    P: TOuterProgram<TAncestry = A>,
{
    type CB: ConstraintBlock<'a, <D as FlowBuilderBase<U>>::T, B::OuterType, U, P>;

    fn get_relevant_builders(topline_constraint_names: &LinkedHashSet<AString>) -> AVec<B> {
        let mut visited = HashSet::new();
        let mut relevant_builders = LinkedHashMap::new();
        let mut g: LinkedHashMap<AString, LinkedHashSet<AString>> = LinkedHashMap::new();
        let mut rev: HashMap<AString, AVec<AString>> = HashMap::new();

        for start in topline_constraint_names {
            let mut builders = B::builders()
                .into_iter()
                .map(|x| (x.get_constraint_name(), x))
                .collect::<LinkedHashMap<AString, _>>();

            let constraint = builders
                .remove(start)
                .expect(format!("Missing constraint named {}", start).as_str());
            let mut builder_q = vec![(start.clone(), constraint)]
                .into_iter()
                .collect::<VecDeque<_>>();

            while builder_q.len() > 0 {
                let (key, builder) = builder_q.pop_front().unwrap();
                let edges = g.entry(key.clone()).or_insert(LinkedHashSet::new());
                debug!("Constraint {} requires:", key);
                for req in builder.get_required_constraint_names() {
                    debug!("  - {}", req);
                    if !visited.contains(&req) {
                        let another = match builders.remove(&req) {
                            Some(x) => x,
                            None => panic!("Cannot find {} in builders.", req),
                        };
                        builder_q.push_back((req.clone(), another));
                        visited.insert(req.clone());
                    }
                    edges.insert(req.clone());
                    let rev_edges = rev.entry(req.clone()).or_insert(AVec::new());
                    rev_edges.push(key.clone());
                }
                relevant_builders.insert(key.clone(), builder);
            }
        }
        let mut sorted_builders = AVec::new();
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
    fn init_unsatisfied_constraints(&self) -> Result<ConstraintsBlockMap<'a, B::OuterType, P>>;

    fn find_satisfiable_constraint_block(
        &self,
        unsatisfied_constraints: &mut ConstraintsBlockMap<'a, B::OuterType, P>,
    ) -> Option<(
        LinkedHashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>>,
        AString,
    )> {
        debug!(
            "There are {} unsatisfied constraints.",
            unsatisfied_constraints.len()
        );
        let constraint_block_name = unsatisfied_constraints
            .iter()
            .filter(|(_, v)| v.0.len() == 0)
            .map(|(k, _)| k.clone())
            .next();
        match constraint_block_name {
            Some(name) => {
                let (_dependency_names, constraints) =
                    unsatisfied_constraints.remove(&name).unwrap();
                debug!(
                    "Found satisfiable constraint block with name {} and size {}",
                    name,
                    constraints.len()
                );
                for (_, (v, _)) in unsatisfied_constraints.iter_mut() {
                    v.remove(&name);
                }
                Some((constraints, name.as_str().into()))
            }
            None => None,
        }
    }
    fn init_tasks_dict(
        block: &LinkedHashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>>,
        constraint_name: AString,
    ) -> AOption<AST> {
        match block.len() == 1 {
            true => AOption(ROption::RNone),
            false => AOption(ROption::RSome(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                format!("tasks_{}", constraint_name).as_str().into(),
            )))),
        }
    }
    fn get_constraint_rwlock(&self, uuid: &(Uuid, AString)) -> RArc<RRwLock<B::OuterType>>;
    fn get_preferences(&self) -> AVec<Dialect>;
    fn get_ancestry(&self) -> &A;
    fn process_constraint_with_program(
        &mut self,
        constraint: RReadGuard<'_, B::OuterType>,
        uuid: (Uuid, AString),
        calls: &mut HashMap<(AString, AString, AString), AVec<(AString, ParameterTuple)>>,
        state: RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        programs: &AVec<P>,
    ) {
        let name = constraint.get_name().clone();
        drop(constraint);
        // TODO: turn into a reference to a field on self
        let preferences = self.get_preferences();
        let mut write = state.write();
        // TODO: remove dummy hash map
        write.satisfy(&preferences, self.get_ancestry(), programs);
        drop(write);

        // TODO: preambles and calls are superflous
        if let AOption(ROption::RSome(key)) = state.read().key.as_ref() {
            calls
                .entry((state.read().get_call().unwrap(), name, uuid.1.clone()))
                .or_insert(AVec::new())
                .push((key.clone(), state.read().get_params().unwrap()))
        } else {
            panic!("No key found for constraint state: {:?}", uuid);
        };
    }
    fn process_constraint_state(
        &mut self,
        uuid: (Uuid, AString),
        state: RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        calls: &mut HashMap<(AString, AString, AString), AVec<(AString, ParameterTuple)>>,
        reverse_dependencies: &HashMap<(Uuid, AString), HashSet<(AString, Uuid, AString)>>,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, B::OuterType, P>,
        programs: &AVec<P>,
    ) -> Result<()> {
        let read = state.read();
        assert!(!read.satisfied);
        assert_eq!(read.unsatisfied_dependencies.len(), 0);
        drop(read);

        let rw = self.get_constraint_rwlock(&uuid);
        let constraint = rw.read();

        if constraint.requires_program()? {
            self.process_constraint_with_program(
                constraint,
                uuid.clone(),
                calls,
                state.clone(),
                programs,
            );
        }

        if let Some(v) = reverse_dependencies.get(&uuid) {
            for (dependency_name, dependency_uuid, dependency_root_type) in v {
                let rw = unsatisfied_constraints
                    .get(dependency_name)
                    .unwrap()
                    .1
                    .get(&(*dependency_uuid, dependency_root_type.clone()))
                    .unwrap();
                let mut write = rw.write();
                write.mark_dependency_as_satisfied(&state, &uuid);
                drop(write);
            }
        }

        let mut write = state.write();
        write.satisfied = true;
        drop(write);
        Ok(())
    }
    fn mark_constraint_state_as_satisfied(
        &mut self,
        id: (Uuid, AString),
        state: RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
    );
    fn process_constraint_block(
        &mut self,
        block: &mut LinkedHashMap<
            (Uuid, AString),
            RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        >,
        reverse_dependencies: &HashMap<(Uuid, AString), HashSet<(AString, Uuid, AString)>>,
        constraint_name: AString,
        unsatisfied_constraints: &ConstraintsBlockMap<'a, B::OuterType, P>,
        identifiers: &mut HashMap<Uuid, AST>,
        programs: &AVec<P>,
        existing_names: &mut HashSet<AString>,
    ) -> Result<(
        AVec<
            <Self::CB as ConstraintBlock<'a, <D as FlowBuilderBase<U>>::T, B::OuterType, U, P>>::C,
        >,
        AOption<AST>,
    )> {
        debug!("Processing constraint block: {}", constraint_name);

        /* TODO: this could be done once for the entire set of blocks
        let to_shorten_constraint_block_names = self
            .get_blocks()
            .iter()
            .map(|x| x.get_constraint_name())
            .chain(vec![constraint_name.clone()].into_iter())
            .collect();
        let shortened_names =
            TaskNameShortener::new(
                to_shorten_constraint_block_names,
                "_".into(),
                HashSet::new()
            ).run();
        let shortened_name = shortened_names.into_iter().last().unwrap();
        debug!("Shortened constraint block name: {}", shortened_name);
        */
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(AString, AString, AString), AVec<(AString, ParameterTuple)>> =
            HashMap::new();
        let mut blocks = AVec::new();
        let mut by_dialect: HashMap<AOption<Dialect>, AVec<_>> = HashMap::new();
        for (id, state) in block.clone() {
            let mut write = state.write();

            write.compute_task_key();
            drop(write);

            self.process_constraint_state(
                id.clone(),
                state.clone(),
                &mut calls,
                reverse_dependencies,
                unsatisfied_constraints,
                programs,
            )?;
            self.mark_constraint_state_as_satisfied(id.clone(), state.clone());
            by_dialect
                .entry(state.read().get_dialect())
                .or_insert(AVec::new())
                .push((state.clone(), id));
        }
        let mut processed = HashMap::new();
        let mut reduced_block = LinkedHashMap::new();
        for (dialect, satisfied) in by_dialect.into_iter() {
            if dialect.is_some() {
                let mut unique: HashMap<_, AVec<_>> = HashMap::new();
                for (c, id) in satisfied.into_iter() {
                    let key = c.read().get_dedup_key();
                    trace!("Dedup key: {:?}", key);
                    unique.entry(key).or_insert(AVec::new()).push((c, id));
                }
                let mut unique_constraints = AVec::new();
                let mut uuid_mappings: HashMap<Uuid, AVec<Uuid>> = HashMap::new();
                for (_, v) in unique {
                    let mut it = v.into_iter();
                    let first = it.next().unwrap();
                    let uuid = first.0.read().get_constraint_uuid().unwrap();
                    reduced_block.insert(first.1.clone(), first.0.clone());
                    unique_constraints.push(first.0);
                    uuid_mappings.insert(uuid, AVec::new());
                    while let Some((elem, _)) = it.next() {
                        let elem_uuid = elem.read().get_constraint_uuid().unwrap();
                        trace!("Inserted Uuid mapping: {} -> {}", &uuid, &elem_uuid);
                        uuid_mappings.get_mut(&uuid).unwrap().push(elem_uuid);
                    }
                }
                processed.insert(dialect, (unique_constraints, uuid_mappings));
            } else {
                processed.insert(
                    dialect,
                    (
                        satisfied.iter().map(|(c, _)| c.clone()).collect(),
                        satisfied
                            .iter()
                            .map(|(c, _id)| {
                                (
                                    c.read().get_constraint_uuid().unwrap(),
                                    vec![c.read().get_constraint_uuid().unwrap()]
                                        .into_iter()
                                        .collect(),
                                )
                            })
                            .collect(),
                    ),
                );
                for elem in satisfied.iter() {
                    reduced_block.insert(elem.1.clone(), elem.0.clone());
                }
            }
        }
        ConstraintState::shorten_task_names(&reduced_block, existing_names);
        let tasks_dict = match processed.values().map(|x| x.0.len()).sum::<usize>() == 1 {
            true => AOption(ROption::RNone),
            false => AOption(ROption::RSome(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                format!("tasks_{}", constraint_name).as_str().into(),
            )))),
        };
        for (_dialect, (unique_constraints, uuid_mappings)) in processed.into_iter() {
            let block = <Self::CB as ConstraintBlock<
                'a,
                <D as FlowBuilderBase<U>>::T,
                B::OuterType,
                U,
                P,
            >>::C::new(
                //satisfied,
                unique_constraints,
                constraint_name.clone(),
                //shortened_name.clone(),
                tasks_dict.clone(),
                identifiers,
                self.get_render_dependencies(),
            )?;
            for (key, val) in block.get_identifiers() {
                for mapped_key in uuid_mappings.get(&key).unwrap().iter() {
                    identifiers.insert(mapped_key.clone(), val.clone());
                }
                identifiers.insert(key, val);
            }
            blocks.push(block);
        }

        Ok((blocks, tasks_dict))
    }
    fn get_render_dependencies(&self) -> bool;
    fn get_constraint_explanation(
        &self,
        constraint_name: &AString,
    ) -> (AOption<AString>, AOption<AString>);
    fn add_block(&mut self, constraint_block: Self::CB);
    fn satisfy_constraints(&mut self) -> Result<()> {
        let mut unsatisfied_constraints = self.init_unsatisfied_constraints()?;
        let mut reverse_dependencies: HashMap<(Uuid, AString), HashSet<(AString, Uuid, AString)>> =
            HashMap::new();
        for (name, (_, constraints)) in &unsatisfied_constraints {
            for ((uuid, root_type), state) in constraints {
                for (dependency_uuid, dependency_root_type) in
                    &state.read().unsatisfied_dependencies
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
                debug!(
                    "Processing constraint {} with block size {}.",
                    constraint_name,
                    block.len()
                );
                let programs = self.get_programs_for(constraint_name);
                let snake_case_name = to_snake_case(constraint_name.as_str().into());
                if block.len() > 0 {
                    let (members, tasks_dict) = self.process_constraint_block(
                        &mut block.clone(),
                        &reverse_dependencies,
                        snake_case_name.as_str().into(),
                        &unsatisfied_constraints,
                        &mut identifiers,
                        &programs,
                        &mut existing_names,
                    )?;

                    let (title, body) = self.get_constraint_explanation(constraint_name);
                    let constraint_block = Self::CB::new(
                        snake_case_name.as_str().into(),
                        title.and_then(|x| ROption::RSome(x.as_str().into())),
                        body.and_then(|x| ROption::RSome(x.as_str().into())),
                        members,
                        tasks_dict,
                    );
                    /*for (key, val) in constraint_block.get_identifiers() {
                        identifiers.insert(key, val);
                    }*/
                    self.add_block(constraint_block);
                }
            } else {
                assert_eq!(unsatisfied_constraints.len(), 0);
                return Ok(());
            }
        }
    }
    fn get_programs_for(&self, constraint_name: &AString) -> AVec<P>;
    fn get_endpoints(&self) -> U::TEndpoints;
    fn get_dependencies(&self) -> AVec<AString>;
    fn run(&mut self, flow_name: AOption<AString>) -> Result<(AString, AVec<AString>)> {
        self.satisfy_constraints()?;
        let etl = D::new();
        let endpoints = self.get_endpoints().clone();
        let statements_and_preambles = self
            .get_blocks()
            .iter()
            .map(|x| x.get_statements(endpoints.clone()))
            .collect::<AVec<_>>();

        Ok((
            etl.materialize(statements_and_preambles, flow_name)?,
            self.get_dependencies(),
        ))
    }
    fn get_blocks(&self) -> &AVec<Self::CB>;
    fn _new(
        concepts: RArc<RRwLock<HashMap<(Uuid, AString), C>>>,
        constraints: LinkedHashMap<(Uuid, AString), RArc<RRwLock<B::OuterType>>>,
        ancestry: A,
        endpoints: <U as AoristUniverse>::TEndpoints,
        ancestors: HashMap<(Uuid, AString), AVec<AncestorRecord>>,
        topline_constraint_names: LinkedHashSet<AString>,
        programs: LinkedHashMap<AString, AVec<P>>,
        preferences: AVec<Dialect>,
        render_dependencies: bool,
    ) -> Self;

    fn generate_constraint_states_map(
        constraints: &LinkedHashMap<(Uuid, AString), RArc<RRwLock<B::OuterType>>>,
        concepts: RArc<
            RRwLock<
                HashMap<
                    (Uuid, AString),
                    <<B::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        ancestors: &HashMap<(Uuid, AString), AVec<AncestorRecord>>,
    ) -> Result<LinkedHashMap<(Uuid, AString), RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>>>
    {
        let mut states_map = LinkedHashMap::new();
        debug!(
            "Generating constraint states map from constraints with size: {}.",
            constraints.len()
        );
        for (k, rw) in constraints {
            debug!(
                "Inserted constraint {} in constraint states map.",
                rw.read().get_name()
            );
            states_map.insert(
                k.clone(),
                RArc::new(RRwLock::new(ConstraintState::new(
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
            (Uuid, AString),
            RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        >,
    ) {
        /* Remove redundant dependencies */
        // constraint key => constraint key dependency on it
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let mut reverse_dependencies: LinkedHashMap<(Uuid, AString), AVec<(Uuid, AString)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                for dep in v.read().unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(AVec::new())
                        .push(k.clone());
                }
            }

            let tips = raw_unsatisfied_constraints
                .iter()
                .filter(|(k, _v)| !reverse_dependencies.contains_key(k));
            for tip in tips {
                let mut visits: HashMap<(Uuid, AString), (Uuid, AString)> = HashMap::new();
                let mut queue: VecDeque<((Uuid, AString), RArc<RRwLock<_>>)> = VecDeque::new();
                queue.push_back((tip.0.clone(), tip.1.clone()));
                while queue.len() > 0 {
                    let (key, elem) = queue.pop_front().unwrap();
                    let new_deps = elem.read().unsatisfied_dependencies.clone();
                    for dep in new_deps {
                        let dep_constraint = raw_unsatisfied_constraints.get(&dep).unwrap().clone();
                        // we have already visited this dependency
                        if visits.contains_key(&dep) {
                            // this is the key of the constraint through which we have already visited
                            let prev = visits.remove(&dep).unwrap();
                            if prev != key {
                                let prev_constraint =
                                    raw_unsatisfied_constraints.get(&prev).unwrap();
                                let mut write = prev_constraint.write();
                                if write.get_name() != elem.read().get_name() {
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
            (Uuid, AString),
            RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        >,
    ) -> Result<()> {
        /* Remove superfluous dummy tasks */
        loop {
            let mut superfluous = AVec::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                let x = v.read();
                if x.unsatisfied_dependencies.len() == 1 && !(x.requires_program()?) {
                    superfluous.push(k.clone());
                }
            }
            if let Some(elem) = superfluous.into_iter().next() {
                let mut reverse_dependencies: LinkedHashMap<
                    (Uuid, AString),
                    AVec<(Uuid, AString)>,
                > = LinkedHashMap::new();
                for (k, v) in raw_unsatisfied_constraints.iter() {
                    for dep in v.read().unsatisfied_dependencies.iter() {
                        reverse_dependencies
                            .entry(dep.clone())
                            .or_insert(AVec::new())
                            .push(k.clone());
                    }
                }
                let arc = raw_unsatisfied_constraints.remove(&elem).unwrap();
                let dep = arc
                    .read()
                    .unsatisfied_dependencies
                    .iter()
                    .next()
                    .unwrap()
                    .clone();

                if let Some(rev_deps) = reverse_dependencies.get(&elem) {
                    for rev in rev_deps.iter() {
                        let mut write = raw_unsatisfied_constraints.get(rev).unwrap().write();
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
            (Uuid, AString),
            RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        >,
    ) -> Result<()> {
        /* Remove dangling dummy tasks */
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let mut dangling = AVec::new();
            let mut reverse_dependencies: LinkedHashMap<(Uuid, AString), AVec<(Uuid, AString)>> =
                LinkedHashMap::new();
            for (k, v) in raw_unsatisfied_constraints.iter() {
                let x = v.read();
                if x.unsatisfied_dependencies.len() == 0 && !(x.requires_program()?) {
                    dangling.push(k.clone());
                }

                trace!("Reverse dependencies for {:?}", k);
                for dep in x.unsatisfied_dependencies.iter() {
                    reverse_dependencies
                        .entry(dep.clone())
                        .or_insert(AVec::new())
                        .push(k.clone());
                    trace!("- {:?}", dep);
                }
            }
            for k in dangling {
                assert!(raw_unsatisfied_constraints.remove(&k).is_some());
                trace!("Dangling: {:?}", k);
                if let Some(v) = reverse_dependencies.get(&k) {
                    for rev in v.iter() {
                        assert!(raw_unsatisfied_constraints
                            .get(rev)
                            .unwrap()
                            .write()
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
        constraints: &LinkedHashMap<(Uuid, AString), RArc<RRwLock<B::OuterType>>>,
        concepts: RArc<
            RRwLock<
                HashMap<
                    (Uuid, AString),
                    <<B::OuterType as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
                >,
            >,
        >,
        ancestors: &HashMap<(Uuid, AString), AVec<AncestorRecord>>,
        _topline_constraint_names: LinkedHashSet<AString>,
    ) -> Result<ConstraintsBlockMap<'a, B::OuterType, P>> {
        let mut raw_unsatisfied_constraints: LinkedHashMap<
            (Uuid, AString),
            RArc<RRwLock<ConstraintState<'a, B::OuterType, P>>>,
        > = Self::generate_constraint_states_map(constraints, concepts, ancestors)?;
        //Self::remove_redundant_dependencies(&mut raw_unsatisfied_constraints);
        Self::remove_superfluous_dummy_tasks(&mut raw_unsatisfied_constraints)?;
        Self::remove_dangling_dummy_tasks(&mut raw_unsatisfied_constraints)?;

        let mut unsatisfied_constraints: LinkedHashMap<_, _> = <<B::OuterType as OuterConstraint<
            'a,
        >>::TEnum as TConstraintEnum<'a>>::get_required_constraint_names(
        )
        .into_iter()
        .map(|(k, v)| (k, (v.into_iter().collect(), LinkedHashMap::new())))
        .collect();

        for ((uuid, root_type), rw) in raw_unsatisfied_constraints.into_iter() {
            let constraint_name = rw.read().get_name();
            unsatisfied_constraints
                .get_mut(&constraint_name)
                .unwrap()
                .1
                .insert((uuid, root_type), rw);
        }

        Ok(unsatisfied_constraints)
    }
    fn get_concept_map_by_object_type(
        concept_map: HashMap<(Uuid, AString), C>,
    ) -> HashMap<AString, AVec<C>> {
        let mut by_object_type: HashMap<AString, AVec<C>> = HashMap::new();
        debug!("Found the following concepts:");
        for ((uuid, object_type), concept) in concept_map {
            debug!("- {}: {}", &object_type, &uuid);
            by_object_type
                .entry(object_type)
                .or_insert(AVec::new())
                .push(concept.clone());
        }
        by_object_type
    }
    fn compute_all_ancestors(
        universe: C,
        concept_map: &HashMap<(Uuid, AString), C>,
    ) -> HashMap<(Uuid, AString), AVec<AncestorRecord>> {
        let mut ancestors: HashMap<(Uuid, AString), AVec<AncestorRecord>> = HashMap::new();
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
                AOption(ROption::RNone),
                0,
            )]
            .into_iter()
            .collect(),
        );
        while frontier.len() > 0 {
            let mut new_frontier: Vec<AncestorRecord> = Vec::new();
            for child in frontier.drain(0..) {
                let key = child.get_key();
                trace!("Ancestors for key: {:?}", key);
                let concept = concept_map.get(&key).unwrap();
                let child_ancestors = ancestors.get(&key).unwrap().clone();
                for grandchild in concept.get_child_concepts() {
                    let t = AncestorRecord::new(
                        grandchild.get_uuid(),
                        grandchild.get_type(),
                        grandchild.get_tag(),
                        grandchild.get_index_as_child(),
                    );
                    trace!("- {:?}", t);
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
        universe: U,
        topline_constraint_names: LinkedHashSet<AString>,
        programs: LinkedHashMap<AString, AVec<P>>,
        preferences: AVec<Dialect>,
        render_dependencies: bool,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let endpoints = universe.get_endpoints();
        let sorted_builders = Self::get_relevant_builders(&topline_constraint_names);
        let mut concept_map: HashMap<(Uuid, AString), C> = HashMap::new();
        let concept = C::from_universe(universe);
        concept.populate_child_concept_map(&mut concept_map);
        let by_object_type = Self::get_concept_map_by_object_type(concept_map.clone());

        let ancestors = Self::compute_all_ancestors(concept, &concept_map);
        let mut visited_constraint_names: LinkedHashSet<AString> = LinkedHashSet::new();
        // constraint_name => root_id => constraint_object
        let mut generated_constraints: LinkedHashMap<
            AString,
            LinkedHashMap<(Uuid, AString), RArc<RRwLock<B::OuterType>>>,
        > = LinkedHashMap::new();

        let concepts = RArc::new(RRwLock::new(concept_map));
        let ancestry: A = A::new(concepts.clone());
        let family_trees = Self::generate_family_trees(&ancestors);

        for builder in sorted_builders.iter() {
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
                constraints.insert((rw.read().get_uuid()?.clone(), root_type), rw.clone());
            }
        }
        debug!("There are {} generated_constraints.", constraints.len());
        Ok(Self::_new(
            concepts,
            constraints,
            ancestry,
            endpoints,
            ancestors,
            topline_constraint_names,
            programs,
            preferences,
            render_dependencies,
        ))
    }
    fn generate_family_trees(
        ancestors: &HashMap<(Uuid, AString), AVec<AncestorRecord>>,
    ) -> HashMap<(Uuid, AString), HashMap<AString, HashSet<Uuid>>> {
        let mut family_trees: HashMap<(Uuid, AString), HashMap<AString, HashSet<Uuid>>> =
            HashMap::new();
        for (key, ancestor_v) in ancestors.iter() {
            for record in ancestor_v.iter() {
                family_trees
                    .entry(key.clone())
                    .or_insert(HashMap::new())
                    .entry(record.object_type.clone())
                    .or_insert(HashSet::new())
                    .insert(record.uuid);
            }
            for record in ancestor_v.iter() {
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
        by_object_type: &HashMap<AString, AVec<C>>,
        family_trees: &HashMap<(Uuid, AString), HashMap<AString, HashSet<Uuid>>>,
        ancestry: &A,
        generated_constraints: &mut LinkedHashMap<
            AString,
            LinkedHashMap<(Uuid, AString), RArc<RRwLock<B::OuterType>>>,
        >,
        visited_constraint_names: &mut LinkedHashSet<AString>,
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

            for root in root_concepts.iter() {
                let root_key = (root.get_uuid(), root.get_type());
                let family_tree = family_trees.get(&root_key).unwrap();
                if builder.should_add(root.clone(), &ancestry) {
                    let raw_potential_child_constraints = builder
                        .get_required_constraint_names()
                        .into_iter()
                        .map(|req| (req.clone(), generated_constraints.get(&req)))
                        .filter(|(_req, x)| x.is_some())
                        .map(|(req, x)| (req, x.unwrap()))
                        .collect::<AVec<_>>();
                    if level_enabled!(Level::DEBUG) {
                        debug!(
                        "Creating constraint {:?} on root {:?} with potential child constraints:",
                        builder.get_constraint_name(),
                        &root_key
                    );
                        for (required_constraint_name, map) in
                            raw_potential_child_constraints.iter()
                        {
                            debug!(" - for {}:", required_constraint_name);
                            for (key, v) in map.iter() {
                                let downstream = v.read();
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
                    if other_required_concept_uuids.len() > 0 {
                        trace!(
                            "Found {} other required concept uuids for root {:?}",
                            other_required_concept_uuids.len(),
                            root.get_uuid()
                        );
                    }
                    let potential_child_constraints = raw_potential_child_constraints
                        .into_iter()
                        .map(|(_req, x)| {
                            x.iter()
                                .filter(
                                    |((potential_root_id, potential_root_type), _constraint)| {
                                        (match family_tree.get(potential_root_type) {
                                            None => false,
                                            Some(set) => set.contains(potential_root_id),
                                        } || other_required_concept_uuids
                                            .contains(potential_root_id))
                                    },
                                )
                                .map(|(_, constraint)| constraint.clone())
                        })
                        .flatten()
                        .collect::<AVec<RArc<RRwLock<B::OuterType>>>>();
                    if level_enabled!(Level::DEBUG) {
                        debug!("After filtering:",);
                        for downstream_rw in potential_child_constraints.iter() {
                            let downstream = downstream_rw.read();
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
                            let downstream = downstream_rw.read();
                            debug!(" --  {:?}", (downstream.get_uuid()?, downstream.get_name()));
                        }
                    }
                    gen_for_constraint.insert(root_key, RArc::new(RRwLock::new(constraint)));
                } else {
                    debug!("Constraint was filtered out.");
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
