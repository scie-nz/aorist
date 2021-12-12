use crate::compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct XMLEncoding {
    #[constrainable]
    pub compression: AOption<AoristRef<DataCompression>>,
}
