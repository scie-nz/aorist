use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct MinioConfig {
    pub server: String,
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
