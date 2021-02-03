#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct AlluxioConfig {
    server: String,
    #[py_default = "19999"]
    rpcPort: usize,
    #[py_default = "39999"]
    apiPort: usize,
}
