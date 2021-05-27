use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;

#[aorist]
pub struct RemoteStorage {
    #[constrainable]
    pub location: RemoteLocation,
    #[constrainable]
    pub layout: APIOrFileLayout,
    #[constrainable]
    pub encoding: Encoding,
}
