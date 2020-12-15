#![allow(non_snake_case)]

use crate::attributes::Attribute;
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::query::{SQLInsertQuery, SQLQuery};
use aorist_concept::Constrainable;
use aorist_primitives::{TAttribute, TOrcAttribute, TPrestoAttribute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
pub struct KeyedStruct {
    name: String,
    #[constrainable]
    attributes: Vec<Attribute>,
}
impl KeyedStruct {
    pub fn get_presto_query(&self) -> SQLInsertQuery {
        let query = SQLInsertQuery::empty();
        query
    }
    pub fn mutate_presto_insert_query(&self, query: &mut SQLInsertQuery) {
        // TODO: handle these with thiserror
        query.set_table_name(self.name.clone()).unwrap();
        query.set_columns(&self.attributes).unwrap();
    }
    fn get_mapped_attributes(&self) -> HashMap<String, Attribute> {
        self.attributes
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
    }
    pub fn get_presto_schema(&self, attributeNames: &Vec<String>) -> Result<String, String> {
        let mapped_attributes = self.get_mapped_attributes();
        let mut schemas: Vec<String> = Vec::new();
        let max_attribute_length = attributeNames.iter().map(|x| x.len()).max().unwrap();
        for attr in attributeNames {
            if mapped_attributes.contains_key(attr) {
                schemas.push(mapped_attributes[attr].get_presto_schema(max_attribute_length))
            } else {
                let err: String = format!(
                    "Cannot find attribute {} in datumTemplate attributes.",
                    attr
                );
                return Err(err);
            }
        }
        Ok(schemas.join(",\n"))
    }
    pub fn get_orc_schema(&self, attributeNames: &Vec<String>) -> Result<String, String> {
        let mapped_attributes = self.get_mapped_attributes();
        let mut schemas: Vec<String> = Vec::new();
        for attr in attributeNames {
            if mapped_attributes.contains_key(attr) {
                schemas.push(mapped_attributes[attr].get_orc_schema())
            } else {
                let err: String = format!(
                    "Cannot find attribute {} in datumTemplate attributes.",
                    attr
                );
                return Err(err);
            }
        }
        Ok(schemas.join(","))
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
