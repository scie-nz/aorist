use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    pub rpc_port: usize,
    pub api_port: usize,
    pub directory: String,
}
