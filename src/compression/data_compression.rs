#![allow(non_snake_case)]
use crate::compression::gzip_compression::*;
use crate::compression::zip_compression::*;
use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
    #[constrainable]
    ZipCompression(ZipCompression),
}
