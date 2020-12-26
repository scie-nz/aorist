use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, AoristConstraint, Constraint};
use crate::data_setup::ParsedDataSetup;
use crate::object::TAoristObject;
use aorist_primitives::{Bash, Dialect, Python};
use indoc::formatdoc;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use uuid::Uuid;

struct ConstraintState {
    _uuid: Uuid,
    _root_type: String,
    name: String,
    satisfied: bool,
    satisfied_dependencies: Vec<(Uuid, String)>,
    unsatisfied_dependencies: HashSet<(Uuid, String)>,
}
impl ConstraintState {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct Driver<'a> {
    _data_setup: &'a ParsedDataSetup,
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
    constraints: HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
    // map from: constraint_name => (dependent_constraint_names, constraints_by_uuid)
    unsatisfied_constraints: HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
        ),
    >,
    concept_ancestors: HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>>,
    ancestry: Arc<ConceptAncestry<'a>>,
}

impl<'a> Driver<'a> {
    fn compute_all_ancestors(
        parsed_data_setup: Concept<'a>,
        concept_map: &HashMap<(Uuid, String), Concept<'a>>,
    ) -> HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>> {
        let mut ancestors: HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>> =
            HashMap::new();
        let mut frontier: Vec<(Uuid, String, Option<String>, usize)> = Vec::new();
        frontier.push((
            parsed_data_setup.get_uuid(),
            parsed_data_setup.get_type(),
            parsed_data_setup.get_tag(),
            parsed_data_setup.get_index_as_child(),
        ));
        ancestors.insert(
            (parsed_data_setup.get_uuid(), parsed_data_setup.get_type()),
            Vec::new(),
        );
        while frontier.len() > 0 {
            let mut new_frontier: Vec<(Uuid, String, Option<String>, usize)> = Vec::new();
            for child in frontier.drain(0..) {
                let concept = concept_map
                    .get(&(child.0.clone(), child.1.clone()))
                    .unwrap();
                let mut grandchild_ancestors = ancestors
                    .get(&(child.0.clone(), child.1.clone()))
                    .unwrap()
                    .clone();
                grandchild_ancestors.push(child.clone());
                for grandchild in concept.get_child_concepts() {
                    new_frontier.push((
                        grandchild.get_uuid(),
                        grandchild.get_type(),
                        grandchild.get_tag(),
                        grandchild.get_index_as_child(),
                    ));
                    ancestors.insert(
                        (grandchild.get_uuid(), grandchild.get_type()),
                        grandchild_ancestors.clone(),
                    );
                }
            }
            frontier = new_frontier;
        }
        ancestors
    }

    pub fn new(data_setup: &'a ParsedDataSetup) -> Driver<'a> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::ParsedDataSetup((data_setup, 0, None));
        concept.populate_child_concept_map(&mut concept_map);

        let constraints = data_setup.get_constraints_map();
        let ancestors = Self::compute_all_ancestors(concept, &concept_map);

        let raw_unsatisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>> =
            constraints
                .iter()
                .map(|(k, rw)| {
                    let x = rw.read().unwrap();
                    let dependencies = x
                        .get_downstream_constraints()
                        .iter()
                        .map(|x| (x.read().unwrap().get_uuid(), x.read().unwrap().root.clone()))
                        .collect::<HashSet<_>>();
                    (
                        k.clone(),
                        Arc::new(RwLock::new(ConstraintState {
                            _uuid: k.0.clone(),
                            _root_type: k.1.clone(),
                            name: x.get_name().clone(),
                            satisfied: false,
                            satisfied_dependencies: Vec::new(),
                            unsatisfied_dependencies: dependencies,
                        })),
                    )
                })
                .collect();

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
        let concepts = Arc::new(RwLock::new(concept_map));
        let ancestry: ConceptAncestry<'a> = ConceptAncestry {
            parents: concepts.clone(),
        };
        Self {
            _data_setup: data_setup,
            concepts,
            constraints: constraints.clone(),
            satisfied_constraints: HashMap::new(),
            unsatisfied_constraints,
            concept_ancestors: ancestors,
            ancestry: Arc::new(ancestry),
        }
    }

    fn find_satisfiable_constraint_block(
        &mut self,
    ) -> Option<HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>> {
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
                Some(constraints)
            }
            None => None,
        }
    }

    fn process_constraint_with_program(
        &mut self,
        constraint: RwLockReadGuard<'_, Constraint>,
        uuid: (Uuid, String),
        preambles: &mut HashSet<String>,
        calls: &mut HashMap<(String, String, String), Vec<(String, String)>>,
    ) {
        let root_uuid = constraint.get_root_uuid();
        let guard = self.concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), constraint.root.clone()))
            .unwrap();

        let ancestors = self
            .concept_ancestors
            .get(&(root_uuid, constraint.root.clone()))
            .unwrap();
        let key = match root.get_tag() {
            None => {
                let mut relative_path: String = "".to_string();
                for (_, ancestor_type, tag, ix) in ancestors.iter().rev() {
                    if let Some(t) = tag {
                        relative_path = format!("{}_of_{}", relative_path, t);
                        break;
                    }
                    if *ix > 0 {
                        relative_path = format!("{}_of_{}_{}", relative_path, ancestor_type, ix);
                    }
                }
                format!("{}{}", root.get_type(), relative_path)
            }
            Some(t) => t,
        };
        let preferences = vec![Dialect::Python(Python {}), Dialect::Bash(Bash {})];
        let ancestry = self.ancestry.clone();
        let root_clone = root.clone();
        let (preamble, call, params) = constraint
            .satisfy_given_preference_ordering(root_clone, &preferences, ancestry)
            .unwrap();
        preambles.insert(preamble);
        calls
            .entry((call, constraint.get_name().clone(), uuid.1.clone()))
            .or_insert(Vec::new())
            .push((key, params));
    }
    fn process_constraint_state(
        &mut self,
        uuid: (Uuid, String),
        state: Arc<RwLock<ConstraintState>>,
        preambles: &mut HashSet<String>,
        calls: &mut HashMap<(String, String, String), Vec<(String, String)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        let rw = self.constraints.get(&uuid).unwrap().clone();
        let constraint = rw.read().unwrap();
        if constraint.requires_program() {
            self.process_constraint_with_program(constraint, uuid.clone(), preambles, calls);
        }
        let read = state.read().unwrap();
        assert!(!read.satisfied);
        assert_eq!(read.unsatisfied_dependencies.len(), 0);
        drop(read);

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
                write.satisfied_dependencies.push(uuid.clone());
                write.unsatisfied_dependencies.remove(&uuid);
                drop(write);
            }
        }

        let mut write = state.write().unwrap();
        write.satisfied = true;
        drop(write);

        self.satisfied_constraints.insert(uuid, state.clone());
    }

    fn process_constraint_block(
        &mut self,
        block: &mut HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        let mut preambles: HashSet<String> = HashSet::new();
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, String)>> = HashMap::new();

        for (id, state) in block.clone() {
            self.process_constraint_state(
                id,
                state,
                &mut preambles,
                &mut calls,
                reverse_dependencies,
            );
        }
        print!(
            "{}\n\n",
            preambles.into_iter().collect::<Vec<String>>().join("\n\n")
        );
        if calls.len() > 0 {
            for ((call, constraint_name, _root_name), params) in calls {
                println!(
                    "{}",
                    formatdoc!(
                        "
                    params_{constraint} = {{
                        {params}
                    }}


                    for k, v in params_{constraint}.items():
                        tasks[k] = {call}(*v)
                    ",
                        constraint = constraint_name,
                        params = params
                            .iter()
                            .map(|(k, v)| format!("'{k}': ({v})", k = k, v = v).to_string())
                            .collect::<Vec<String>>()
                            .join(",\n    "),
                        call = call,
                    )
                );
            }
        }
    }
    pub fn run(&mut self) {
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
            if let Some(ref mut block) = satisfiable {
                //println!("Block has size: {}", block.len());
                self.process_constraint_block(&mut block.clone(), &reverse_dependencies);
            } else {
                break;
            }
        }
        assert_eq!(self.unsatisfied_constraints.len(), 0);
    }
}
