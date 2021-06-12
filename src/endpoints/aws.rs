#![allow(non_snake_case)]
use crate::constraint::Constraint;
use crate::{AoristConcept, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct AWSConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
