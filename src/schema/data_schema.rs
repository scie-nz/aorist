#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::schema::tabular_schema::*;
use crate::schema::undefined_tabular_schema::*;
use crate::schema::time_ordered_tabular_schema::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum DataSchema {
    #[constrainable]
    TabularSchema(TabularSchema),
    #[constrainable]
    TimeOrderedTabularSchema(TimeOrderedTabularSchema),
    #[constrainable]
    UndefinedTabularSchema(UndefinedTabularSchema),
}

impl DataSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            DataSchema::TabularSchema(x) => Ok(x.datumTemplateName.clone()),
            DataSchema::TimeOrderedTabularSchema(x) => Ok(x.datumTemplateName.clone()),
            DataSchema::UndefinedTabularSchema(_) => {
                Err("UndefinedTabularSchema has no datum template.".to_string())
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::TabularSchema(x) => x.attributes.clone(),
            DataSchema::TimeOrderedTabularSchema(x) => x.attributes.clone(),
            DataSchema::UndefinedTabularSchema(_) => vec![],
        }
    }
}
impl InnerDataSchema {
    pub fn get_datum_template_name(&self) -> PyResult<String> {
        match self {
            InnerDataSchema::TabularSchema(x) => Ok(x.datumTemplateName.clone()),
            InnerDataSchema::TimeOrderedTabularSchema(x) => Ok(x.datumTemplateName.clone()),
            InnerDataSchema::UndefinedTabularSchema(_) => {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "UndefinedTabularSchema has no datum template.".to_string(),
                ))
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            InnerDataSchema::TabularSchema(x) => x.attributes.clone(),
            InnerDataSchema::TimeOrderedTabularSchema(x) => x.attributes.clone(),
            InnerDataSchema::UndefinedTabularSchema(_) => vec![],
        }
    }
}
