#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use crate::compressions::DataCompression;
use crate::headers::FileHeader;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CSVEncoding {
    compression: DataCompression,
    header: FileHeader,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content="spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
}
