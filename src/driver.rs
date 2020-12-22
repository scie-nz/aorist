use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::data_setup::ParsedDataSetup;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

struct ConstraintState {
    uuid: Uuid,
    satisfied: bool,
    satisfied_dependencies: Vec<Uuid>,
    unsatisfied_dependencies: HashSet<Uuid>,
}

pub struct Driver<'a> {
    data_setup: &'a ParsedDataSetup,
    concepts: HashMap<Uuid, Concept<'a>>,
    constraints: HashMap<Uuid, Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<Uuid, Arc<RwLock<ConstraintState>>>,
    unsatisfied_constraints: HashMap<Uuid, Arc<RwLock<ConstraintState>>>,
}

impl<'a> Driver<'a> {
    pub fn new(data_setup: &'a ParsedDataSetup) -> Driver<'a> {
        let mut concept_map: HashMap<Uuid, Concept<'a>> = HashMap::new();
        let concept = Concept::ParsedDataSetup(data_setup);
        concept.populate_child_concept_map(&mut concept_map);
        let constraints = data_setup.get_constraints_map();

        Self {
            data_setup,
            concepts: concept_map,
            constraints: constraints.clone(),
            satisfied_constraints: HashMap::new(),
            unsatisfied_constraints: constraints
                .iter()
                .map(|(k, rw)| {
                    let x = rw.read().unwrap();
                    let dependencies = x
                        .get_downstream_constraints()
                        .iter()
                        .map(|x| x.read().unwrap().get_uuid())
                        .collect::<HashSet<Uuid>>();
                    (
                        k.clone(),
                        Arc::new(RwLock::new(ConstraintState {
                            uuid: k.clone(),
                            satisfied: false,
                            satisfied_dependencies: Vec::new(),
                            unsatisfied_dependencies: dependencies,
                        })),
                    )
                })
                .collect(),
        }
    }

    fn find_satisfiable_constraint(&self) -> Option<Uuid> {
        self
        .unsatisfied_constraints
        .iter()
        .filter(|(_, v)| v.read().unwrap().unsatisfied_dependencies.len() == 0)
        .map(|(k, _)| k.clone())
        .next()
    }
    pub fn run(&mut self) {
        let mut reverse_dependencies: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
        for (uuid, state) in &self.unsatisfied_constraints {
            for dependency_uuid in &state.read().unwrap().unsatisfied_dependencies {
                reverse_dependencies
                    .entry(*dependency_uuid)
                    .or_insert(HashSet::new())
                    .insert(*uuid);
            }
        }

        let mut satisfiable = self.find_satisfiable_constraint();
        // find at least one satisfiable constraint
        while let Some(uuid) = satisfiable {
            let state = self.unsatisfied_constraints.remove(&uuid).unwrap();
            let constraint = self.constraints.get(&uuid).unwrap().read().unwrap();
            if constraint.requires_program() {
                println!("{} requires program.", uuid);
            }
            let read = state.read().unwrap();
            assert!(!read.satisfied);
            assert_eq!(read.unsatisfied_dependencies.len(), 0);
            for dep in &read.satisfied_dependencies {
                println!("{} -> {}", dep, uuid);
            }
            drop(read);

            if let Some(v) = reverse_dependencies.get(&uuid) {
                for dependency_uuid in v {
                    let rw = self.unsatisfied_constraints.get(dependency_uuid).unwrap();
                    let mut write = rw.write().unwrap();
                    write.unsatisfied_dependencies.remove(&uuid);
                    write.satisfied_dependencies.push(uuid);
                    drop(write);
                }
            }

            let mut write = state.write().unwrap();
            write.satisfied = true;
            drop(write);

            self.satisfied_constraints.insert(uuid, state);
            satisfiable = self.find_satisfiable_constraint();
        }
        assert_eq!(self.unsatisfied_constraints.len(), 0);
    }
}
