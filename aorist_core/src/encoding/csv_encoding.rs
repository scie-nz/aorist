use crate::compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct CSVEncoding {
    #[constrainable]
    pub compression: Option<AoristRef<DataCompression>>,
    #[constrainable]
    pub header: Option<AoristRef<FileHeader>>,
}
