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
pub struct UpperSnakeCaseCSVHeader {
    pub num_lines: Option<usize>,
}
#[pymethods]
impl UpperSnakeCaseCSVHeader {
    #[new]
    #[args(num_lines = "None")]
    fn new(num_lines: Option<usize>) -> Self {
        Self {
            num_lines,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
