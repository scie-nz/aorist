#![allow(non_snake_case)]
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
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    #[py_default = "19998"]
    pub rpcPort: usize,
    #[py_default = "39999"]
    pub apiPort: usize,
    pub directory: String,
}
