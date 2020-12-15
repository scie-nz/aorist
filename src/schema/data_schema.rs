#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::schema::tabular_schema::TabularSchema;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum DataSchema {
    #[constrainable]
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
