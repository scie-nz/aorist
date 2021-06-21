use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
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
