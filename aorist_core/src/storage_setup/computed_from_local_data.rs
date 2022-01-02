use aorist_primitives::AoristRef;
use crate::concept::WrappedConcept;
use crate::storage::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConceptBase, AoristConcept, ConceptEnum};
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
