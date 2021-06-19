use crate::compression::gzip_compression::*;
use crate::compression::zip_compression::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
    #[constrainable]
    ZipCompression(ZipCompression),
}
