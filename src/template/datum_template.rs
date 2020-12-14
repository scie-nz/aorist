#![allow(non_snake_case)]

use aorist_concept::Constrainable;
use crate::concept::AoristConcept;
use crate::template::keyed_struct::KeyedStruct;
use serde::{Deserialize, Serialize};

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
