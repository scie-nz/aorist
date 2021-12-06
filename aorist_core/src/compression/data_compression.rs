use crate::compression::bzip2_compression::*;
use crate::compression::gzip_compression::*;
use crate::compression::laz_compression::*;
use crate::compression::zip_compression::*;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum DataCompression {
    #[constrainable]
    BZip2Compression(AoristRef<BZip2Compression>),
    #[constrainable]
    GzipCompression(AoristRef<GzipCompression>),
    #[constrainable]
    ZipCompression(AoristRef<ZipCompression>),
    #[constrainable]
    LAZCompression(AoristRef<LAZCompression>),
}
