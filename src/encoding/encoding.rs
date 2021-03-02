#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, Concept};
use crate::encoding::csv_encoding::*;
use crate::encoding::onnx_encoding::*;
use crate::encoding::orc_encoding::*;
use crate::encoding::tsv_encoding::*;
use crate::header::FileHeader;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
    TSVEncoding(TSVEncoding),
    ONNXEncoding(TSVEncoding),
}

impl Encoding {
    pub fn get_header(&self) -> Option<FileHeader> {
        match &self {
            Self::CSVEncoding(x) => x.header.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.header.clone()),
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
        }
    }
    pub fn get_compression(&self) -> Option<DataCompression> {
        match &self {
            Self::CSVEncoding(x) => x.compression.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.compression.clone()),
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
        }
    }
}
