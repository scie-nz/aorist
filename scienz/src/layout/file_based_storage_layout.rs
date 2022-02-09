use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
