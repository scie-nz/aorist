use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct GCSLocation {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub blob: String,
}
