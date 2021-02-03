#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::user::{TUser, User};
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
    users: Option<Vec<User>>,
}
#[pymethods]
impl UserGroup {
    #[new]
    #[args(labels = "HashMap::new()", description = "None")]
    fn new(
        name: String,
        labels: HashMap<String, String>,
        description: Option<String>,
        users: Vec<User>,
    ) -> Self {
        Self {
            name,
            members: users.iter().map(|x| x.get_unixname()).collect(),
            labels,
            description,
            users: Some(users),
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
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
