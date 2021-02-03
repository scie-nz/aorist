#![allow(non_snake_case)]
use crate::compression::gzip_compression::GzipCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
}
