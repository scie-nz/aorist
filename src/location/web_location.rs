use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::constraint::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct WebLocation {
    // TODO: replace these with Getters and Setters
    pub address: String,
}
