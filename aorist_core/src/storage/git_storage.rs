use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct GitStorage {
    #[constrainable]
    pub location: AoristRef<GithubLocation>,
    #[constrainable]
    layout: AoristRef<FileBasedStorageLayout>,
    #[constrainable]
    pub encoding: AoristRef<Encoding>,
}
