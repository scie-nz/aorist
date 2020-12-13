#![allow(non_snake_case)]

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::encoding::csv_encoding::CSVEncoding;
use crate::encoding::orc_encoding::ORCEncoding;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
