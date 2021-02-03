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
pub struct AlluxioConfig {
    server: String,
    rpcPort: usize,
    apiPort: usize,
}

#[pymethods]
impl AlluxioConfig {
    #[new]
    #[args(rpcPort = "19999", apiPort = "39999")]
    fn new(server: String, rpcPort: usize, apiPort: usize) -> Self {
        Self {
            server,
            rpcPort,
            apiPort,
            tag: None,
            constraints: Vec::new(),
            uuid: None,
        }
    }
}
