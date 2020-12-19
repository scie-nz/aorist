use crate::concept::AoristConcept;
use crate::object::TAoristObject;
use aorist_primitives::{define_constraint, register_constraint};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub trait TConstraint
where
    Self::Root: AoristConcept,
{
    type Root;
    fn get_root_type_name() -> String;
    fn get_required_constraint_names() -> Vec<String>;
}

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: String,
    pub root: String,
    pub requires: Option<Vec<String>>,
}
impl Constraint {
    pub fn get_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_uuid();
        }
        panic!("Called get_uuid() on a Constraint struct with no inner");
    }
    pub fn get_root(&self) -> String {
        self.root.clone()
    }
    pub fn get_root_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_root_uuid();
        }
        panic!("Called get_root_uuid() on a Constraint struct with no inner");
    }
    pub fn get_downstream_constraints(&self) -> Vec<Rc<Constraint>> {
        if let Some(c) = &self.inner {
            return c.get_downstream_constraints();
        }
        panic!("Called get_downstream_constraints() on a Constraint struct with no inner");
    }
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
