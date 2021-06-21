use crate::storage::*;
use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct LocalStorageSetup {
    #[constrainable]
    pub local: AoristRef<Storage>,
    pub tmp_dir: String,
}
