#![allow(non_snake_case)]
//use crate::ranger::RangerEntity;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::object::TAoristObject;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGroupRangerPayload {
    name: String,
    description: String,
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &String {
        &self.name
    }
}
