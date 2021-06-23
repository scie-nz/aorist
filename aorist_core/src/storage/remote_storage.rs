use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct RemoteStorage {
    #[constrainable]
    pub location: AoristRef<RemoteLocation>,
    #[constrainable]
    pub layout: AoristRef<APIOrFileLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
