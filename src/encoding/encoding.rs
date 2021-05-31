#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::encoding::csv_encoding::*;
use crate::encoding::gdb_encoding::*;
use crate::encoding::json_encoding::*;
use crate::encoding::onnx_encoding::*;
use crate::encoding::orc_encoding::*;
use crate::encoding::tsv_encoding::*;
use crate::header::FileHeader;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    JSONEncoding(JSONEncoding),
    NewlineDelimitedJSONEncoding(JSONEncoding),
    ORCEncoding(ORCEncoding),
    TSVEncoding(TSVEncoding),
    ONNXEncoding(TSVEncoding),
    GDBEncoding(TSVEncoding),
}

impl Encoding {
    pub fn get_header(&self) -> Option<FileHeader> {
        match &self {
            Self::CSVEncoding(x) => x.header.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.header.clone()),
            Self::JSONEncoding(_) => None,
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
            Self::GDBEncoding(_) => None,
            Self::NewlineDelimitedJSONEncoding(_) => None,
        }
    }
    pub fn get_compression(&self) -> Option<DataCompression> {
        match &self {
            Self::CSVEncoding(x) => x.compression.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.compression.clone()),
            Self::GDBEncoding(x) => x.compression.clone(),
            Self::JSONEncoding(_) => None,
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
            Self::NewlineDelimitedJSONEncoding(_) => None,
        }
    }
    pub fn get_default_file_extension(&self) -> String {
        match &self {
            Self::CSVEncoding(_) => "csv".to_string(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(_) => "tsv".to_string(),
            Self::GDBEncoding(_) => "gdb".to_string(),
            Self::JSONEncoding(_) => "json".to_string(),
            Self::ORCEncoding(_) => "orc".to_string(),
            Self::ONNXEncoding(_) => "onnx".to_string(),
            Self::NewlineDelimitedJSONEncoding(_) => "json".to_string(),
        }
    }
}
