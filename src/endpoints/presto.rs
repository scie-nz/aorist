#![allow(non_snake_case)]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::Concept;
use aorist_concept::{aorist_concept, Constrainable};
use crate::AoristConcept;
use crate::constraint::Constraint;
use derivative::Derivative;
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
        Self { server, httpPort,
            tag: None,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}
