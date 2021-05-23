use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct GCSLocation {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub blob: String,
}
