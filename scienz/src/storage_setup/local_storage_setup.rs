use crate::storage::*;
use crate::storage_setup::two_tier_storage_setup::*;
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
pub struct LocalStorageSetup {
    #[constrainable]
    pub local: AoristRef<Storage>,
    pub tmp_dir: AString,
}
impl LocalStorageSetup {
    pub fn persist(&self, persistent: AoristRef<Storage>) -> TwoTierStorageSetup {
        TwoTierStorageSetup {
            scratch: self.local.clone(),
            persistent,
            tmp_dir: self.tmp_dir.clone(),
            tag: self.tag.clone(),
            uuid: AOption(ROption::RNone),
        }
    }
}
