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
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}

#[pymethods]
impl GiteaConfig {
    #[new]
    #[args(server = "\"localhost\".to_string()", port = "30807")]
    fn new(server: String, port: usize, token: String) -> Self {
        Self {
            server,
            port,
            token,
            tag: None,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}
