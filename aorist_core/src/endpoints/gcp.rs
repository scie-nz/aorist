use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct GCPConfig {
    pub use_default_credentials: bool,
    pub service_account_file: Option<String>,
    pub project_name: String,
    pub data_location: String,
}
