#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct PrestoConfig {
    server: String,
    httpPort: usize,
}

#[pymethods]
impl PrestoConfig {
    #[new]
    #[args(httpPort = "8080")]
    fn new(server: String, httpPort: usize) -> Self {
        Self {
            server,
            httpPort,
            tag: None,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}
