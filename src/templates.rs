#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use crate::attributes::{Attribute, TAttribute, TPrestoAttribute};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyedStruct {
    name: String, attributes: Vec<Attribute>
}
impl KeyedStruct {
    pub fn get_presto_schema(&self) -> String {
        let max_attribute_length = self.attributes.iter().map(|x| x.get_name().len()).max().unwrap();
        self.attributes.iter().map(|x| x.get_presto_schema(max_attribute_length)).collect::<Vec<String>>().join(",\n")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
}
impl DatumTemplate {
    pub fn get_presto_schema(&self) -> String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_presto_schema(),
        }
    }
}
