use crate::compression::gzip_compression::GzipCompression;
use crate::compression::zip_compression::ZipCompression;
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
