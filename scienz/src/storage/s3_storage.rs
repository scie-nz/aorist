use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use aorist_primitives::AUuid;

#[aorist]
pub struct S3Storage {
    #[constrainable]
    pub location: AoristRef<S3Location>,
    #[constrainable]
    pub layout: AoristRef<FileBasedStorageLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
