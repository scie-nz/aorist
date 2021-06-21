use crate::compression::*;
use crate::header::*;
use crate::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct CSVEncoding {
    #[constrainable]
    pub compression: Option<AoristRef<DataCompression>>,
    #[constrainable]
    pub header: Option<AoristRef<FileHeader>>,
}
