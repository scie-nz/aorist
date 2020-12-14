#![allow(non_snake_case)]

use crate::encoding::csv_encoding::CSVEncoding;
use crate::encoding::orc_encoding::ORCEncoding;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
