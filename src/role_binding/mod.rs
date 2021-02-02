#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::role::Role;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct RoleBinding {
    user_name: String,
    #[constrainable]
    role: Role,
    name: String,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl RoleBinding {
    pub fn get_user_name(&self) -> &String {
        &self.user_name
    }
    pub fn get_role(&self) -> &Role {
        &self.role
    }
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
