use crate::concept::Concept;
use crate::constraint::{AllConstraintsSatisfiability, Constraint, SatisfiableConstraint, AoristConstraint, TConstraint};
use crate::data_setup::ParsedDataSetup;
use crate::object::TAoristObject;
use aorist_primitives::{Dialect, Python};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

struct ConstraintState {
    uuid: Uuid,
    root_type: String,
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
    data_setup: &'a ParsedDataSetup,
    concepts: HashMap<(Uuid, String), Concept<'a>>,
    constraints: HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>,
    // map from: constraint_name => (dependent_constraint_names, constraints_by_uuid)
    unsatisfied_constraints:
        HashMap<String, (HashSet<String>, HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>)>,
}

impl<'a> Driver<'a> {
    pub fn new(data_setup: &'a ParsedDataSetup) -> Driver<'a> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::ParsedDataSetup(data_setup);
        concept.populate_child_concept_map(&mut concept_map);
        let constraints = data_setup.get_constraints_map();

        let raw_unsatisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>> = constraints
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
                        uuid: k.0.clone(),
                        root_type: k.1.clone(),
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
            (HashSet<String>, HashMap<(Uuid, String), Arc<RwLock<ConstraintState>>>),
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
        Self {
            data_setup,
            concepts: concept_map,
            constraints: constraints.clone(),
            satisfied_constraints: HashMap::new(),
            unsatisfied_constraints,
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
                let (dependency_names, constraints) =
                    self.unsatisfied_constraints.remove(&name).unwrap();
                for (k, (v, _)) in self.unsatisfied_constraints.iter_mut() {
                    v.remove(&name);
                }
                Some(constraints)
            }
            None => None,
        }
    }
    pub fn run(&mut self) {
        let mut reverse_dependencies: HashMap<(Uuid, String), HashSet<(String, Uuid, String)>> = HashMap::new();
        for (name, (_, constraints)) in &self.unsatisfied_constraints {
            for ((uuid, root_type), state) in constraints {
                for (dependency_uuid, dependency_root_type) in &state.read().unwrap().unsatisfied_dependencies {
                    reverse_dependencies
                        .entry((*dependency_uuid, dependency_root_type.clone()))
                        .or_insert(HashSet::new())
                        .insert((name.clone(), *uuid, root_type.clone()));
                }
            }
        }

        let mut satisfiable = self.find_satisfiable_constraint_block();
        // find at least one satisfiable constraint
        while let Some(ref mut block) = satisfiable {
            println!("Block has size: {}", block.len());
            for (uuid, state) in block.clone() {
                let rw = self.constraints.get(&uuid).unwrap().clone();
                let constraint = rw.read().unwrap();
                println!("Processing {}({}) {}.", &uuid.0, &uuid.1,  constraint.get_name());
                if constraint.requires_program() {
                    println!("{}({}) requires program.", &uuid.0, &uuid.1);
                    let root_uuid = constraint.get_root_uuid();
                    let root = self
                        .concepts
                        .get(&(root_uuid, constraint.root.clone()))
                        .unwrap();
                    let preferences = vec![Dialect::Python(Python {})];
                    let out = constraint
                        .satisfy_given_preference_ordering(root, &preferences)
                        .unwrap();
                    println!("Parameters: {}", out.2);
                    println!("Call: {}", out.1);
                    println!("Preamble: {}", out.0);
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
                            .unwrap().1
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
            satisfiable = self.find_satisfiable_constraint_block();
        }
        assert_eq!(self.unsatisfied_constraints.len(), 0);
    }
}
