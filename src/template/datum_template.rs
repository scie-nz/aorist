#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use crate::template::keyed_struct::KeyedStruct;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
