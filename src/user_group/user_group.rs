#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable)]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
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
