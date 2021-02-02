#![allow(non_snake_case)]
use crate::compression::gzip_compression::GzipCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type")]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
}
