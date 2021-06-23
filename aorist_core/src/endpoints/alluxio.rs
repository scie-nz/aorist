use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    pub rpc_port: usize,
    pub api_port: usize,
    pub directory: String,
}
