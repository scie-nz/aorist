#![allow(non_snake_case)]
use crate::schema::tabular_schema::TabularSchema;
use crate::template::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum DataSchema {
    TabularSchema(TabularSchema),
}
impl DataSchema {
    pub fn get_presto_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        match self {
            DataSchema::TabularSchema(x) => x.get_presto_schema(templates),
        }
    }
    pub fn get_orc_schema(&self, templates: &HashMap<String, DatumTemplate>) -> String {
        match self {
            DataSchema::TabularSchema(x) => x.get_orc_schema(templates),
        }
    }
}
