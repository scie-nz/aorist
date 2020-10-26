#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GzipCompression {}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataCompression {
    GzipCompression(GzipCompression),
}
