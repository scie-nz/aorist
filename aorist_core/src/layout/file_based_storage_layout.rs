use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[aorist]
pub struct SingleFileLayout {}

#[aorist]
pub enum FileBasedStorageLayout {
    SingleFileLayout(SingleFileLayout),
}
