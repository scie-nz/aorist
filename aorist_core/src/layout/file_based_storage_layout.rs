use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct SingleFileLayout {}

#[aorist]
pub struct CompressedFileCollectionLayout {}

#[aorist]
pub struct DirectoryLayout {
    max_num_files: AOption<usize>,
}

#[aorist]
pub enum FileBasedStorageLayout {
    SingleFileLayout(AoristRef<SingleFileLayout>),
    DirectoryLayout(AoristRef<DirectoryLayout>),
    CompressedFileCollectionLayout(AoristRef<CompressedFileCollectionLayout>),
}
