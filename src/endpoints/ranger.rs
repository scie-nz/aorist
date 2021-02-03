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
