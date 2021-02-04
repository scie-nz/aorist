use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
use aorist_concept::{aorist_concept, InnerObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct GiteaConfig {
    #[py_default = "\"localhost\".to_string()"]
    server: String,
    #[py_default = "30807"]
    port: usize,
    token: String,
}
