use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist(derivative(Hash))]
pub struct AWSConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
