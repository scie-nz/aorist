use aorist_concept::{aorist, Constrainable};
use crate::{AoristConcept, ConceptEnum};
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::compression::gzip_compression::GzipCompression;
use crate::compression::zip_compression::ZipCompression;

#[aorist]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
    #[constrainable]
    ZipCompression(ZipCompression),
}
