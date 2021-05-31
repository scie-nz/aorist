use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::layout::api_layout::*;
use crate::layout::file_based_storage_layout::*;

#[aorist]
pub enum APIOrFileLayout {
    FileBasedStorageLayout(FileBasedStorageLayout),
    APILayout(APILayout),
}
