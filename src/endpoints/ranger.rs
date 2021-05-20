use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::{AoristConcept, AoristConceptChildren};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct RangerConfig {
    #[py_default = "\"localhost\".to_string()"]
    server: String,
    #[py_default = "30800"]
    port: usize,
    #[py_default = "\"admin\".to_string()"]
    user: String,
    password: String,
}
