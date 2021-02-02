#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::user::{User, TUser};

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
    users: Option<Vec<User>>,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
#[pymethods]
impl UserGroup {
    #[new]
    #[args(labels="HashMap::new()", description="None")]
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
