use crate::compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use aorist_paste::paste;
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
