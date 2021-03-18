#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::schema::tabular_schema::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum DataSchema {
    #[constrainable]
    TabularSchema(TabularSchema),
}

impl DataSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            DataSchema::TabularSchema(x) => Ok(x.datumTemplateName.clone()),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::TabularSchema(x) => x.attributes.clone(),
        }
    }
}
impl InnerDataSchema {
    pub fn get_datum_template_name(&self) -> PyResult<String> {
        match self {
            InnerDataSchema::TabularSchema(x) => Ok(x.datumTemplateName.clone()),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            InnerDataSchema::TabularSchema(x) => x.attributes.clone(),
        }
    }
}
