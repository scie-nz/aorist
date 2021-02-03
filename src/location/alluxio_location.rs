use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct AlluxioLocation {
    path: String,
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
