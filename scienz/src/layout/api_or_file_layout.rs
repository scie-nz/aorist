use crate::layout::api_layout::*;
use crate::layout::file_based_storage_layout::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub enum APIOrFileLayout {
    FileBasedStorageLayout(AoristRef<FileBasedStorageLayout>),
    APILayout(AoristRef<APILayout>),
}
