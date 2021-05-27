use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::concept::{AoristConcept, ConceptEnum};

#[aorist]
pub struct LocalFileSystemLocation {
    pub path: String,
}
