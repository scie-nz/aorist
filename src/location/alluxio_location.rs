use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct AlluxioLocation {
    path: String,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}

#[pymethods]
impl AlluxioLocation {
    #[new]
    fn new(path: String) -> Self {
        Self {
            path,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
