use crate::code_block::CodeBlock;
use crate::concept::{AoristConcept, Concept, ConceptAncestry};
use crate::constraint::{AoristConstraint, Constraint};
use crate::constraint_block::ConstraintBlock;
use crate::constraint_state::ConstraintState;
use crate::data_setup::{TUniverse, Universe};
use crate::etl_singleton::ETLDAG;
use crate::object::TAoristObject;
use crate::python::ParameterTuple;
use aorist_primitives::{Bash, Dialect, Presto, Python};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use uuid::Uuid;

pub struct Driver<'a, D>
where
    D: ETLDAG,
{
    _data_setup: &'a Universe,
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
    constraints: HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
    blocks: Vec<ConstraintBlock<'a, D::T>>,
    // map from: constraint_name => (dependent_constraint_names, constraints_by_uuid)
    unsatisfied_constraints: HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        ),
    >,
    ancestry: Arc<ConceptAncestry<'a>>,
    dag_type: PhantomData<D>,
}

impl<'a, D> Driver<'a, D>
where
    D: ETLDAG,
{
    fn compute_all_ancestors(
        universe: Concept<'a>,
        concept_map: &HashMap<(Uuid, String), Concept<'a>>,
    ) -> HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>> {
        let mut ancestors: HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>> =
            HashMap::new();
        let mut frontier: Vec<(Uuid, String, Option<String>, usize)> = Vec::new();
        frontier.push((
            universe.get_uuid(),
            universe.get_type(),
            universe.get_tag(),
            universe.get_index_as_child(),
        ));
        ancestors.insert(
            (universe.get_uuid(), universe.get_type()),
            vec![(universe.get_uuid(), universe.get_type(), None, 0)],
        );
        while frontier.len() > 0 {
            let mut new_frontier: Vec<(Uuid, String, Option<String>, usize)> = Vec::new();
            for child in frontier.drain(0..) {
                let concept = concept_map
                    .get(&(child.0.clone(), child.1.clone()))
                    .unwrap();
                let child_ancestors = ancestors
                    .get(&(child.0.clone(), child.1.clone()))
                    .unwrap()
                    .clone();
                for grandchild in concept.get_child_concepts() {
                    let t = (
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
    fn get_unsatisfied_constraints(
        constraints: &HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        ancestors: &HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>>,
    ) -> HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        ),
    > {
        let mut raw_unsatisfied_constraints: HashMap<
            (Uuid, String),
            Arc<RwLock<ConstraintState<'a>>>,
        > = constraints
            .iter()
            .map(|(k, rw)| {
                (
                    k.clone(),
                    Arc::new(RwLock::new(ConstraintState::new(
                        rw.clone(),
                        concepts.clone(),
                        ancestors,
                    ))),
                )
            })
            .collect();

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
        /* Remove dangling dummy tasks */
        let mut changes_made = true;
        while changes_made {
            changes_made = false;
            let dangling = raw_unsatisfied_constraints
                .iter()
                .filter(|(_k, v)| {
                    v.read().unwrap().unsatisfied_dependencies.len() == 0
                        && !v.read().unwrap().requires_program()
                })
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();
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
            for k in dangling {
                assert!(raw_unsatisfied_constraints.remove(&k).is_some());
                for rev in reverse_dependencies.get(&k).unwrap() {
                    assert!(raw_unsatisfied_constraints
                        .get(rev)
                        .unwrap()
                        .write()
                        .unwrap()
                        .unsatisfied_dependencies
                        .remove(&k));
                }
                changes_made = true;
            }
        }
        /* Remove superfluous dummy tasks */
        loop {
            let superfluous = raw_unsatisfied_constraints
                .iter()
                .filter(|(_k, v)| {
                    v.read().unwrap().unsatisfied_dependencies.len() == 1
                        && !v.read().unwrap().requires_program()
                })
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();
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

                for rev in reverse_dependencies.get(&elem).unwrap() {
                    let mut write = raw_unsatisfied_constraints
                        .get(rev)
                        .unwrap()
                        .write()
                        .unwrap();
                    assert!(write.unsatisfied_dependencies.remove(&elem));
                    write.unsatisfied_dependencies.insert(dep.clone());
                }
            } else {
                break;
            }
        }

        let mut unsatisfied_constraints: HashMap<
            String,
            (
                HashSet<String>,
                HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
            ),
        > = AoristConstraint::get_required_constraint_names()
            .into_iter()
            .map(|(k, v)| (k, (v.into_iter().collect(), HashMap::new())))
            .collect();

        for ((uuid, root_type), rw) in raw_unsatisfied_constraints.into_iter() {
            let constraint_name = rw.read().unwrap().get_name();
            unsatisfied_constraints
                .get_mut(&constraint_name)
                .unwrap()
                .1
                .insert((uuid, root_type), rw);
        }

        unsatisfied_constraints
    }
    pub fn new(
        data_setup: &'a Universe,
        topline_constraint_names: Option<HashSet<String>>,
    ) -> Driver<'a, D> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::Universe((data_setup, 0, None));
        concept.populate_child_concept_map(&mut concept_map);

        let it = data_setup.get_constraints().iter().map(|x| x.clone());
        let topline = match topline_constraint_names {
            Some(set) => it
                .filter(|x| {
                    let r = x.read().unwrap();
                    r.root == "Universe" && set.contains(&r.name)
                })
                .collect(),
            None => it.collect(),
        };
        let constraints = data_setup.get_constraints_map(topline);

        let ancestors = Self::compute_all_ancestors(concept, &concept_map);
        let concepts = Arc::new(RwLock::new(concept_map));
        let unsatisfied_constraints =
            Self::get_unsatisfied_constraints(&constraints, concepts.clone(), &ancestors);

        let ancestry: ConceptAncestry<'a> = ConceptAncestry {
            parents: concepts.clone(),
        };
        Self {
            _data_setup: data_setup,
            concepts,
            constraints: constraints.clone(),
            satisfied_constraints: HashMap::new(),
            unsatisfied_constraints,
            ancestry: Arc::new(ancestry),
            blocks: Vec::new(),
            dag_type: PhantomData,
        }
    }

    fn find_satisfiable_constraint_block(
        &mut self,
    ) -> Option<(
        HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        String,
    )> {
        let constraint_block_name = self
            .unsatisfied_constraints
            .iter()
            .filter(|(_, v)| v.0.len() == 0)
            .map(|(k, _)| k.clone())
            .next();
        match constraint_block_name {
            Some(name) => {
                let (_dependency_names, constraints) =
                    self.unsatisfied_constraints.remove(&name).unwrap();
                for (_, (v, _)) in self.unsatisfied_constraints.iter_mut() {
                    v.remove(&name);
                }
                Some((constraints, name))
            }
            None => None,
        }
    }

    fn process_constraint_with_program(
        &mut self,
        constraint: RwLockReadGuard<'_, Constraint>,
        uuid: (Uuid, String),
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        let name = constraint.get_name().clone();
        drop(constraint);
        let preferences = vec![
            Dialect::Python(Python {}),
            Dialect::Presto(Presto {}),
            Dialect::Bash(Bash {}),
        ];

        let mut write = state.write().unwrap();
        // TODO: remove dummy hash map
        write.satisfy(&preferences, self.ancestry.clone());
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
        state: Arc<RwLock<ConstraintState<'a>>>,
        calls: &mut HashMap<(String, String, String), Vec<(String, ParameterTuple)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        let mut write = state.write().unwrap();
        let ancestors = write.get_ancestors();
        write.compute_task_name(&ancestors);
        // TODO: rename this function, it's confusing (this represents
        // constraitn name, key is root name)
        assert!(!write.satisfied);
        assert_eq!(write.unsatisfied_dependencies.len(), 0);
        drop(write);

        let rw = self.constraints.get(&uuid).unwrap().clone();
        let constraint = rw.read().unwrap();
        if constraint.requires_program() {
            self.process_constraint_with_program(constraint, uuid.clone(), calls, state.clone());
        }

        if let Some(v) = reverse_dependencies.get(&uuid) {
            for (dependency_name, dependency_uuid, dependency_root_type) in v {
                let rw = self
                    .unsatisfied_constraints
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
    }

    fn process_constraint_block(
        &mut self,
        block: &mut HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
        constraint_name: String,
    ) -> Vec<CodeBlock<'a, D::T>> {
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, ParameterTuple)>> =
            HashMap::new();
        let mut blocks: Vec<CodeBlock<'a, D::T>> = Vec::new();
        let mut by_dialect: HashMap<Option<Dialect>, Vec<Arc<RwLock<ConstraintState<'a>>>>> =
            HashMap::new();
        for (id, state) in block.clone() {
            self.process_constraint_state(
                id.clone(),
                state.clone(),
                &mut calls,
                reverse_dependencies,
            );
            self.satisfied_constraints.insert(id, state.clone());
            by_dialect
                .entry(state.read().unwrap().get_dialect())
                .or_insert(Vec::new())
                .push(state.clone());
        }
        for (dialect, satisfied) in by_dialect.into_iter() {
            let block = CodeBlock::new(dialect, satisfied, constraint_name.clone());
            blocks.push(block);
        }

        blocks
    }
    fn get_shorter_task_name(task_name: String) -> String {
        let splits = task_name
            .split("__")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();
        let mut new_name = task_name.to_string();
        if splits.len() > 2 {
            new_name = format!(
                "{}__{}",
                splits[0].to_string(),
                splits[2..]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join("__")
            )
            .to_string();
        } else if splits.len() == 2 {
            new_name = splits[0].to_string();
        } else {
            let splits_inner = splits[0]
                .split("_")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if splits_inner.len() > 2 {
                new_name = format!(
                    "{}_{}",
                    splits_inner[0].to_string(),
                    splits_inner[2..]
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()
                        .join("_")
                )
                .to_string();
            }
        }
        new_name
    }
    pub fn shorten_task_names(&mut self) {
        let mut task_names: Vec<(String, Arc<RwLock<ConstraintState<'a>>>)> = Vec::new();
        // shorten task names
        for constraint in self.satisfied_constraints.values() {
            let fqn = constraint.read().unwrap().get_fully_qualified_task_name();
            task_names.push((fqn, constraint.clone()));
        }
        loop {
            let mut changes_made = false;

            let mut proposed_names: Vec<String> = task_names.iter().map(|x| x.0.clone()).collect();
            let mut new_task_names: HashSet<String> = proposed_names.clone().into_iter().collect();
            for i in 0..task_names.len() {
                let task_name = proposed_names.get(i).unwrap().clone();
                let new_name = Self::get_shorter_task_name(task_name.clone());
                if new_name != task_name
                    && !new_task_names.contains(&new_name)
                    && proposed_names
                        .iter()
                        .enumerate()
                        .filter(|(pos, x)| *pos != i && x.contains(&new_name))
                        .collect::<Vec<_>>()
                        .len()
                        == 0
                {
                    changes_made = true;
                    new_task_names.insert(new_name.clone());
                    proposed_names[i] = new_name;
                }
            }
            if !changes_made {
                break;
            }
            for i in 0..task_names.len() {
                task_names[i].0 = proposed_names[i].clone();
            }
        }
        for (name, rw) in task_names {
            let mut write = rw.write().unwrap();
            write.set_task_name(name.replace("____", "__"));
        }
    }
    pub fn run(&'a mut self) -> String {
        let mut reverse_dependencies: HashMap<(Uuid, String), HashSet<(String, Uuid, String)>> =
            HashMap::new();
        for (name, (_, constraints)) in &self.unsatisfied_constraints {
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

        // find at least one satisfiable constraint
        loop {
            let mut satisfiable = self.find_satisfiable_constraint_block();
            if let Some((ref mut block, ref constraint_name)) = satisfiable {
                let snake_case_name = to_snake_case(constraint_name);
                let members = self.process_constraint_block(
                    &mut block.clone(),
                    &reverse_dependencies,
                    snake_case_name.clone(),
                );

                // TODO: this can be moved to ConstraintBlock
                let constraint_block = ConstraintBlock::new(snake_case_name, members);
                self.blocks.push(constraint_block);
            } else {
                break;
            }
        }
        self.shorten_task_names();

        let etl = D::new();
        assert_eq!(self.unsatisfied_constraints.len(), 0);
        let statements_and_preambles = self
            .blocks
            .iter()
            .map(|x| x.get_statements())
            .collect::<Vec<_>>();
        return etl.materialize(statements_and_preambles);
    }
}
