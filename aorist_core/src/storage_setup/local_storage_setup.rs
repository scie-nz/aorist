use crate::storage::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct LocalStorageSetup {
    #[constrainable]
    pub local: AoristRef<Storage>,
    pub tmp_dir: String,
}
