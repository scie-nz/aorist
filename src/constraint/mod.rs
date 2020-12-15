use aorist_primitives::define_constraint;
use crate::concept::AoristConcept;
use crate::object::TAoristObject;
use serde::{Deserialize, Serialize};
use std::fmt;

pub trait TConstraint
where Self::Root: AoristConcept {
    type Root;
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Hash, Eq)]
pub struct Constraint {
    name: String,
    root: String,
    requires: Option<Vec<String>>,
}
impl TAoristObject for Constraint {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));
