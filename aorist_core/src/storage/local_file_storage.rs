use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct LocalFileStorage {
    #[constrainable]
    pub location: AoristRef<OnPremiseLocation>,
    #[constrainable]
    pub layout: AoristRef<FileBasedStorageLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
