#![allow(non_snake_case)]

use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::encoding::csv_encoding::CSVEncoding;
use crate::encoding::orc_encoding::ORCEncoding;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
