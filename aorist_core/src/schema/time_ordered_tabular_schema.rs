#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[aorist]
pub struct TimeOrderedTabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub attributes: Vec<String>,
    // non-null time stamp columns used to order records
    // order is always: 1st column, then 2nd, etc.
    pub orderingAttributes: Vec<String>,
}
impl TimeOrderedTabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyTimeOrderedTabularSchema {
    #[getter]
    pub fn datum_template(&self) -> PyDatumTemplate {
        PyDatumTemplate{ inner: self.inner.0.read().unwrap().get_datum_template().clone() }
    }
}
