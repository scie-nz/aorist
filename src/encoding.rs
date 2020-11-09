#![allow(non_snake_case)]

use crate::compressions::DataCompression;
use crate::headers::FileHeader;
use crate::hive::THiveTableCreationTagMutator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CSVEncoding {
    compression: DataCompression,
    header: FileHeader,
}
impl THiveTableCreationTagMutator for CSVEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        tags.insert("format".to_string(), "CSV".to_string());
        Ok(())
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ORCEncoding {}
impl THiveTableCreationTagMutator for ORCEncoding {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        tags.insert("format".to_string(), "ORC".to_string());
        Ok(())
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
