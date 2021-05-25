#![allow(non_snake_case)]
use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::{AoristConcept, AoristConceptChildren, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct PrestoConfig {
    pub server: String,
    #[py_default = "8080"]
    pub httpPort: usize,
    pub user: String,
}
