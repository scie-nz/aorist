use crate::compression::*;
use crate::header::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct TSVEncoding {
    #[constrainable]
    pub compression: DataCompression,
    #[constrainable]
    pub header: FileHeader,
}
