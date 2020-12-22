use crate::concept::Concept;
use crate::constraint::Constraint;
use std::collections::HashMap;
use uuid::Uuid;
use crate::data_setup::ParsedDataSetup;
use std::sync::{Arc, RwLock};

pub struct Driver<'a> {
    data_setup: &'a ParsedDataSetup,
    concepts: HashMap<Uuid, Concept<'a>>,
    constraints: HashMap<Uuid, Arc<RwLock<Constraint>>>,
}

impl <'a> Driver<'a> {
    pub fn new(
        data_setup: &'a ParsedDataSetup
    ) -> Driver<'a> {

        let mut concept_map: HashMap<Uuid, Concept<'a>> = HashMap::new();
        let concept = Concept::ParsedDataSetup(data_setup);
        concept.populate_child_concept_map(&mut concept_map);
        let constraints = data_setup.get_constraints_map();

        Self {
            data_setup,
            concepts: concept_map,
            constraints,
        }
    }
}
