#![allow(non_snake_case)]
use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::user::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct UserGroup {
    name: String,
    #[py_default = "Vec::new()"]
    members: Vec<String>,
    #[py_default = "HashMap::new()"]
    labels: HashMap<String, String>,
    #[py_default = "None"]
    description: Option<String>,
    #[constrainable]
    #[py_default = "Vec::new()"]
    users: Vec<User>,
}
pub trait TUserGroup {
    fn get_labels(&self) -> &HashMap<String, String>;
}
impl TUserGroup for UserGroup {
    fn get_labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &String {
        &self.name
    }
}
