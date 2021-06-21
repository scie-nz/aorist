use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct MinioConfig {
    pub server: String,
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
