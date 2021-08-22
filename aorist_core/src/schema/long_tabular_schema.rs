use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct LongTabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub key_attributes: Vec<String>,
    pub value_attributes: Vec<String>,
}
impl LongTabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyLongTabularSchema {
    #[getter]
    pub fn datum_template(&self) -> PyDatumTemplate {
        PyDatumTemplate {
            inner: self.inner.0.read().unwrap().get_datum_template().clone(),
        }
    }
}

impl LongTabularSchema {
    pub fn get_attribute_names(&self) -> Vec<String> {
        self.key_attributes
            .clone()
            .into_iter()
            .chain(self.value_attributes.clone().into_iter())
            .collect()
    }
    pub fn should_dedup_text_attribute(&self, attr: &String) -> bool {
        for attribute in &self.key_attributes {
            if attr == attribute {
                return true;
            }
        }
        false
    }
}
