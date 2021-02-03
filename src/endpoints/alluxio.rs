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
