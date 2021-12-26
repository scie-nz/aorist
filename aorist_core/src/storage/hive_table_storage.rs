use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
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
pub struct HiveTableStorage {
    #[constrainable]
    pub location: AoristRef<HiveLocation>,
    #[constrainable]
    layout: AoristRef<TabularLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
