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
pub struct GitStorage {
    #[constrainable]
    pub location: GithubLocation,
    #[constrainable]
    layout: FileBasedStorageLayout,
    #[constrainable]
    pub encoding: Encoding,
}
