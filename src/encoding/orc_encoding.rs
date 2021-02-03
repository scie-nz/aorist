#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use aorist_concept::{aorist_concept, Constrainable};

#[aorist_concept]
pub struct ORCEncoding {}
#[pymethods]
impl ORCEncoding {
    #[new]
    fn new(tag: Option<String>) -> Self {
        Self {
            tag,
            uuid: None,
            constraints: Vec::new(),
        }
    }
}
