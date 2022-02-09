use crate::compression::bzip2_compression::*;
use crate::compression::gzip_compression::*;
use crate::compression::laz_compression::*;
use crate::compression::zip_compression::*;

use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
