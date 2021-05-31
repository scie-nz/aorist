use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist(derivative(Hash))]
pub struct MinioConfig {
    pub server: String,
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
