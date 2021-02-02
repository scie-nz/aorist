use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct GCSLocation {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub blob: String,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}

#[pymethods]
impl GCSLocation {
    #[new]
    fn new(bucket: String, blob: String) -> Self {
        Self {
            bucket,
            blob,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
