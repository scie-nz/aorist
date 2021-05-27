use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist]
pub struct S3Location {
    // TODO: replace these with Getters and Setters
    pub bucket: String,
    pub key: String,
}
