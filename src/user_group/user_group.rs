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

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl UserGroup {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &String {
        &self.name
    }
}
