#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::schema::tabular_schema::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub enum DataSchema {
    #[constrainable]
    TabularSchema(TabularSchema),
}

impl DataSchema {
    pub fn get_datum_template_name(&self) -> String {
        match self {
            DataSchema::TabularSchema(x) => x.datumTemplateName.clone(),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::TabularSchema(x) => x.attributes.clone(),
        }
    }
}
impl InnerDataSchema {
    pub fn get_datum_template_name(&self) -> String {
        match self {
            InnerDataSchema::TabularSchema(x) => x.datumTemplateName.clone(),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            InnerDataSchema::TabularSchema(x) => x.attributes.clone(),
        }
    }
}
