use crate::compression::*;
use crate::header::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TSVEncoding {
    #[constrainable]
    pub compression: AoristRef<DataCompression>,
    #[constrainable]
    pub header: AoristRef<FileHeader>,
}
