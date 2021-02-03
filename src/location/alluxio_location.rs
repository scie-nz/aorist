use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct AlluxioLocation {
    path: String,
}
