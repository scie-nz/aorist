#![allow(non_snake_case)]
use crate::compression::gzip_compression::GzipCompression;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DataCompression {
    GzipCompression(GzipCompression),
}
