use crate::concept::{AoristRef, WrappedConcept};
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
use crate::storage_setup::two_tier_storage_setup::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct LocalStorageSetup {
    #[constrainable]
    pub local: AoristRef<Storage>,
    pub tmp_dir: String,
}
impl LocalStorageSetup {
    pub fn persist(&self, persistent: AoristRef<Storage>) -> TwoTierStorageSetup {
        TwoTierStorageSetup { 
            scratch: self.local.clone(),
            persistent,
            tmp_dir: self.tmp_dir.clone(),
            tag: self.tag.clone(),
            uuid: None,
        }
    }
}
