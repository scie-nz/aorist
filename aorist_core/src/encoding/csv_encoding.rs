use crate::compression::DataCompression;
use crate::header::FileHeader;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct CSVEncoding {
    #[constrainable]
    pub compression: Option<DataCompression>,
    #[constrainable]
    pub header: Option<FileHeader>,
}
