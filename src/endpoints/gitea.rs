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
