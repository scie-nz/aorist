use crate::compression::gzip_compression::*;
use crate::compression::zip_compression::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(AoristRef<GzipCompression>),
    #[constrainable]
    ZipCompression(AoristRef<ZipCompression>),
}
