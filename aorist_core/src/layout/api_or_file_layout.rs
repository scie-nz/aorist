use crate::layout::api_layout::*;
use crate::layout::file_based_storage_layout::*;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum APIOrFileLayout {
    FileBasedStorageLayout(AoristRef<FileBasedStorageLayout>),
    APILayout(AoristRef<APILayout>),
}
