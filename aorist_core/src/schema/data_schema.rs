#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::fasttext_embedding_schema::*;
use crate::schema::tabular_schema::*;
use crate::schema::time_ordered_tabular_schema::*;
use crate::schema::undefined_tabular_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum DataSchema {
    #[constrainable]
    FasttextEmbeddingSchema(AoristRef<FasttextEmbeddingSchema>),
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    TimeOrderedTabularSchema(AoristRef<TimeOrderedTabularSchema>),
    #[constrainable]
    UndefinedTabularSchema(AoristRef<UndefinedTabularSchema>),
}

impl DataSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            DataSchema::FasttextEmbeddingSchema(x) => Ok(x
                .0
                .read()
                .unwrap()
                .source_schema()
                .0
                .read()
                .unwrap()
                .datumTemplateName
                .clone()),
            DataSchema::TabularSchema(x) => Ok(x.0.read().unwrap().datumTemplateName.clone()),
            DataSchema::TimeOrderedTabularSchema(x) => {
                Ok(x.0.read().unwrap().datumTemplateName.clone())
            }
            DataSchema::UndefinedTabularSchema(_) => {
                Err("UndefinedTabularSchema has no datum template.".to_string())
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::FasttextEmbeddingSchema(x) => {
                x.0.read()
                    .unwrap()
                    .source_schema()
                    .0
                    .read()
                    .unwrap()
                    .attributes
                    .clone()
            }
            DataSchema::TabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::TimeOrderedTabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::UndefinedTabularSchema(_) => vec![],
        }
    }
}
#[cfg(feature = "python")]
impl PyDataSchema {
    pub fn get_datum_template_name(&self) -> PyResult<String> {
        match self.inner.0.read().unwrap().get_datum_template_name() {
            Ok(s) => Ok(s),
            Err(err) => Err(PyValueError::new_err(err)),
        }
    }
}
