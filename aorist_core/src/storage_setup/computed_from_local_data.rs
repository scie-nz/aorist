use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::concept::{AoristRef, WrappedConcept};
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ComputedFromLocalData {
    pub source_asset_names: BTreeMap<AString, AString>,
    #[constrainable]
    pub target: AoristRef<Storage>,
    pub tmp_dir: AString,
}
