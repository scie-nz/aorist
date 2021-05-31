use aorist_concept::{aorist, Constrainable};
use crate::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::compression::DataCompression;
use crate::header::FileHeader; 

#[aorist]
pub struct TSVEncoding {
    #[constrainable]
    pub compression: DataCompression,
    #[constrainable]
    pub header: FileHeader,
}
