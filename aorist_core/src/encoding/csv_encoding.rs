use aorist_concept::{aorist, Constrainable};
use crate::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::compression::DataCompression;
use crate::header::FileHeader; 

#[aorist]
pub struct CSVEncoding {
    #[constrainable]
    pub compression: Option<DataCompression>,
    #[constrainable]
    pub header: Option<FileHeader>,
}
