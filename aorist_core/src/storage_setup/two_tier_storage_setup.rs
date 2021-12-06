use crate::concept::{AoristRef, WrappedConcept};
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TwoTierStorageSetup {
    #[constrainable]
    pub scratch: AoristRef<Storage>,
    #[constrainable]
    pub persistent: AoristRef<Storage>,
    pub tmp_dir: AString,
}
