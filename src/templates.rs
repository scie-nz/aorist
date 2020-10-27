#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use crate::attributes::{Attribute, TAttribute, TPrestoAttribute};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyedStruct {
    name: String,
    attributes: Vec<Attribute>,
}
impl KeyedStruct {
    fn get_mapped_attributes(&self) -> HashMap<String, Attribute> {
        self.attributes.iter().map(|x| (x.get_name().clone(), x.clone())).collect()
    }
    pub fn get_presto_schema(&self, attributeNames: &Vec<String>, indent: usize) -> Result<String, String> {
        let mapped_attributes = self.get_mapped_attributes();
        let mut schemas: Vec<String> = Vec::new();
        let max_attribute_length = attributeNames.iter().map(|x| x.len()).max().unwrap();
        for attr in attributeNames {
            if mapped_attributes.contains_key(attr) {
                schemas.push(mapped_attributes[attr].get_presto_schema(indent, max_attribute_length))
            } else {
                let err: String = format!("Cannot find attribute {} in datumTemplate attributes.", attr);
                return Err(err);
            }
        }
        Ok(schemas.join(",\n"))
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
}
impl DatumTemplate {
    pub fn get_presto_schema(&self, attributeNames: &Vec<String>, indent: usize) -> String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_presto_schema(attributeNames, indent).unwrap(),
        }
    }
    pub fn get_name(&self) -> &String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_name(),
        }
    }
}
