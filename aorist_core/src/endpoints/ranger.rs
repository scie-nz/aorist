use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist(derivative(Hash))]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}
