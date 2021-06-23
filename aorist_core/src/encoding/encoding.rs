#![allow(unused_parens)]
use crate::compression::DataCompression;
use crate::concept::{AoristRef, WrappedConcept};
use crate::header::FileHeader;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::encoding::csv_encoding::*;
use crate::encoding::gdb_encoding::*;
use crate::encoding::json_encoding::*;
use crate::encoding::onnx_encoding::*;
use crate::encoding::orc_encoding::*;
use crate::encoding::tsv_encoding::*;
use paste::paste;
use std::fmt::Debug;

#[aorist]
pub enum Encoding {
    CSVEncoding(AoristRef<CSVEncoding>),
    JSONEncoding(AoristRef<JSONEncoding>),
    NewlineDelimitedJSONEncoding(AoristRef<NewlineDelimitedJSONEncoding>),
    ORCEncoding(AoristRef<ORCEncoding>),
    TSVEncoding(AoristRef<TSVEncoding>),
    ONNXEncoding(AoristRef<ONNXEncoding>),
    GDBEncoding(AoristRef<GDBEncoding>),
}

impl Encoding {
    pub fn get_header(&self) -> Option<AoristRef<FileHeader>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().unwrap().header.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.0.read().unwrap().header.clone()),
            Self::JSONEncoding(_) => None,
            Self::ORCEncoding(_) => None,
            Self::ONNXEncoding(_) => None,
            Self::GDBEncoding(_) => None,
            Self::NewlineDelimitedJSONEncoding(_) => None,
        }
    }
    pub fn get_compression(&self) -> Option<AoristRef<DataCompression>> {
        match &self {
            Self::CSVEncoding(x) => x.0.read().unwrap().compression.clone(),
            // TODO: need to change this to also be optional
            Self::TSVEncoding(x) => Some(x.0.read().unwrap().compression.clone()),
            Self::GDBEncoding(x) => x.0.read().unwrap().compression.clone(),
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
