use crate::compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConceptBase, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TSVEncoding {
    #[constrainable]
    pub compression: AOption<AoristRef<DataCompression>>,
    #[constrainable]
    pub header: AOption<AoristRef<FileHeader>>,
}
