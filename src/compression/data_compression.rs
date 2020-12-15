#![allow(non_snake_case)]
use crate::constraint::Constraint;
use crate::compression::gzip_compression::GzipCompression;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum DataCompression {
    #[constrainable]
    GzipCompression(GzipCompression),
}
