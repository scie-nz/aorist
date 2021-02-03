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
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

#[pymethods]
impl RangerConfig {
    #[new]
    #[args(
        server = "\"localhost\".to_string()",
        port = "30800",
        user = "\"admin\".to_string()"
    )]
    fn new(server: String, port: usize, user: String, password: String) -> Self {
        Self {
            server,
            port,
            user,
            password,
            tag: None,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}
