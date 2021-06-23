use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct AWSConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
