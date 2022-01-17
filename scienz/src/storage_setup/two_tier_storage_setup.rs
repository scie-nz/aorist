use crate::storage::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::AoristRef;
use aorist_primitives::{AString, AVec, AoristConcept, AoristConceptBase, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct TwoTierStorageSetup {
    #[constrainable]
    pub scratch: AoristRef<Storage>,
    #[constrainable]
    pub persistent: AoristRef<Storage>,
    pub tmp_dir: AString,
}
