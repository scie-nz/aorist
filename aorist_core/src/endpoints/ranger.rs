use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}
