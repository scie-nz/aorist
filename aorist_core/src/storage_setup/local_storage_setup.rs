use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use crate::storage::*;
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct LocalStorageSetup {
    #[constrainable]
    pub local: Storage,
    pub tmp_dir: String,
}
