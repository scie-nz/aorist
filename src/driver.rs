use crate::concept::{Concept, ConceptAncestry};
use crate::constraint::{AllConstraintsSatisfiability, AoristConstraint, Constraint};
use crate::data_setup::ParsedDataSetup;
use crate::object::TAoristObject;
use aorist_primitives::{Bash, Dialect, Python};
use indoc::formatdoc;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock, RwLockReadGuard};
use uuid::Uuid;

struct ConstraintState<'a> {
    dialect: Option<Dialect>,
    key: Option<String>,
    name: String,
    satisfied: bool,
    satisfied_dependencies: Vec<(Uuid, String)>,
    unsatisfied_dependencies: HashSet<(Uuid, String)>,
    constraint: Arc<RwLock<Constraint>>,
    root: Concept<'a>,
    // these are concept ancestors
    // TODO: change this to Vec<Concept<'a>>
    ancestors: Vec<(Uuid, String, Option<String>, usize)>,
    preamble: Option<String>,
    call: Option<String>,
    params: Option<String>,
}
impl<'a> ConstraintState<'a> {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[allow(dead_code)]
    pub fn get_root(&self) -> Concept<'a> {
        self.root.clone()
    }
    #[allow(dead_code)]
    pub fn get_root_uuid(&self) -> Uuid {
        self.root.get_uuid().clone()
    }
    pub fn get_ancestors(&self) -> Vec<(Uuid, String, Option<String>, usize)> {
        self.ancestors.clone()
    }
    pub fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    pub fn get_params(&self) -> Option<String> {
        self.params.clone()
    }
    pub fn get_call(&self) -> Option<String> {
        self.call.clone()
    }
    fn satisfy(&mut self, preferences: &Vec<Dialect>, ancestry: Arc<ConceptAncestry<'a>>) {
        let root_clone = self.root.clone();
        let constraint = self.constraint.read().unwrap();
        let (preamble, call, params, dialect) = constraint
            .satisfy_given_preference_ordering(root_clone, preferences, ancestry)
            .unwrap();
        self.preamble = Some(preamble);
        self.call = Some(call);
        self.params = Some(params);
        self.dialect = Some(dialect);
    }
    fn new(
        constraint: Arc<RwLock<Constraint>>,
        concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
        concept_ancestors: &HashMap<(Uuid, String), Vec<(Uuid, String, Option<String>, usize)>>,
    ) -> Self {
        let arc = constraint.clone();
        let x = arc.read().unwrap();
        let root_uuid = x.get_root_uuid();
        let guard = concepts.read().unwrap();
        let root = guard
            .get(&(root_uuid.clone(), x.root.clone()))
            .unwrap()
            .clone();
        let dependencies = x
            .get_downstream_constraints()
            .iter()
            .map(|x| (x.read().unwrap().get_uuid(), x.read().unwrap().root.clone()))
            .collect::<HashSet<_>>();
        let ancestors = concept_ancestors
            .get(&(root_uuid, x.root.clone()))
            .unwrap()
            .clone();
        Self {
            dialect: None,
            key: None,
            name: x.get_name().clone(),
            satisfied: false,
            unsatisfied_dependencies: dependencies,
            satisfied_dependencies: Vec::new(),
            constraint,
            root,
            ancestors: ancestors.clone(),
            preamble: None,
            call: None,
            params: None,
        }
    }
    fn compute_task_name(&mut self, ancestors: &Vec<(Uuid, String, Option<String>, usize)>) {
        self.key = Some(match self.root.get_tag() {
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
                format!("{}{}", self.root.get_type(), relative_path)
            }
            Some(t) => t,
        });
    }
}

pub struct Driver<'a> {
    _data_setup: &'a ParsedDataSetup,
    pub concepts: Arc<RwLock<HashMap<(Uuid, String), Concept<'a>>>>,
    constraints: HashMap<(Uuid, String), Arc<RwLock<Constraint>>>,
    satisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
    blocks: Vec<Vec<Arc<RwLock<ConstraintState<'a>>>>>,
    // map from: constraint_name => (dependent_constraint_names, constraints_by_uuid)
    unsatisfied_constraints: HashMap<
        String,
        (
            HashSet<String>,
            HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        ),
    >,
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
        let raw_unsatisfied_constraints: HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>> =
            constraints
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
    pub fn new(data_setup: &'a ParsedDataSetup) -> Driver<'a> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::ParsedDataSetup((data_setup, 0, None));
        concept.populate_child_concept_map(&mut concept_map);

        let constraints = data_setup.get_constraints_map();
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
        }
    }

    fn find_satisfiable_constraint_block(
        &mut self,
    ) -> Option<HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>> {
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
        calls: &mut HashMap<(String, String, String), Vec<(String, String)>>,
        state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        let ancestors = state.read().unwrap().get_ancestors();

        let preferences = vec![Dialect::Python(Python {}), Dialect::Bash(Bash {})];

        let mut write = state.write().unwrap();
        write.satisfy(&preferences, self.ancestry.clone());
        write.compute_task_name(&ancestors);
        drop(write);

        // TODO: preambles and calls are superflous
        let key = state.read().unwrap().key.as_ref().unwrap().clone();
        calls
            .entry((
                state.read().unwrap().get_call().unwrap(),
                constraint.get_name().clone(),
                uuid.1.clone(),
            ))
            .or_insert(Vec::new())
            .push((key, state.read().unwrap().get_params().unwrap()));
    }
    fn process_constraint_state(
        &mut self,
        uuid: (Uuid, String),
        state: Arc<RwLock<ConstraintState<'a>>>,
        calls: &mut HashMap<(String, String, String), Vec<(String, String)>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        let rw = self.constraints.get(&uuid).unwrap().clone();
        let constraint = rw.read().unwrap();
        if constraint.requires_program() {
            self.process_constraint_with_program(constraint, uuid.clone(), calls, state.clone());
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
    }

    fn process_constraint_block(
        &mut self,
        block: &mut HashMap<(Uuid, String), Arc<RwLock<ConstraintState<'a>>>>,
        reverse_dependencies: &HashMap<(Uuid, String), HashSet<(String, Uuid, String)>>,
    ) {
        // (call, constraint_name, root_name) => (uuid, call parameters)
        let mut calls: HashMap<(String, String, String), Vec<(String, String)>> = HashMap::new();

        let mut satisfied: Vec<Arc<RwLock<ConstraintState<'a>>>> = Vec::new();
        for (id, state) in block.clone() {
            self.process_constraint_state(
                id.clone(),
                state.clone(),
                &mut calls,
                reverse_dependencies,
            );
            self.satisfied_constraints.insert(id, state.clone());
            satisfied.push(state.clone());
        }
        self.blocks.push(satisfied);
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
        let preambles: HashSet<String> = self
            .blocks
            .iter()
            .map(|x| x.iter().map(|y| y.read().unwrap().get_preamble()))
            .flatten()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        print!(
            "{}\n\n",
            preambles.into_iter().collect::<Vec<String>>().join("\n\n")
        );
        assert_eq!(self.unsatisfied_constraints.len(), 0);
    }
}
