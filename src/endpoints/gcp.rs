#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct GCPConfig {
    pub use_default_credentials: bool,
    #[py_default = "None"]
    pub service_account_file: Option<String>,
    pub project_name: String,
    #[py_default = "\"US\".to_string()"]
    pub data_location: String,
}