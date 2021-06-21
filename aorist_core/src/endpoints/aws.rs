use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct AWSConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
