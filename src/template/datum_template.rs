#![allow(non_snake_case)]

use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::template::keyed_struct::KeyedStruct;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
}
impl DatumTemplate {
    pub fn get_presto_schema(&self, attributeNames: &Vec<String>) -> String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_presto_schema(attributeNames).unwrap(),
        }
    }
    pub fn get_orc_schema(&self, attributeNames: &Vec<String>) -> String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_orc_schema(attributeNames).unwrap(),
        }
    }
    pub fn get_name(&self) -> &String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_name(),
        }
    }
}
