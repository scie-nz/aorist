use crate::concept::AoristConcept;
use crate::object::TAoristObject;
use aorist_primitives::{define_constraint, register_constraint};
use serde::{Deserialize, Serialize};
use std::fmt;
use maplit::hashmap;
use std::collections::HashMap;

pub trait TConstraint
where
    Self::Root: AoristConcept,
{
    type Root;
    fn get_root_type_name() -> String;
    fn get_required_constraint_names() -> Vec<String>;
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
