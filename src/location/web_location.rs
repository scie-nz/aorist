use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct WebLocation {
    // TODO: replace these with Getters and Setters
    pub address: String,
}
#[pymethods]
impl WebLocation {
    #[new]
    fn new(address: String) -> Self {
        Self {
            address,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
