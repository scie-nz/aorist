#![allow(non_snake_case)]

use crate::compressions::DataCompression;
use crate::headers::FileHeader;
use crate::hive::THiveTableCreationTagMutator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::python::TObjectWithPythonCodeGen;
use aorist_derive::{BlankPrefectPreamble, NoPythonImports};
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectEncoding, TPrefectCompression};

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
impl TObjectWithPythonCodeGen for CSVEncoding {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.compression.get_python_imports(preamble)
    }
}
impl TObjectWithPrefectCodeGen for CSVEncoding {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.compression.get_prefect_preamble(preamble)
    }
}
impl TPrefectEncoding for CSVEncoding {
    fn get_prefect_decode_tasks(
        &self,
        file_name: String,
        task_name: String,
        upstream_task_name: String,
    ) -> String {
        self.compression.get_prefect_decompress_task(
            file_name, task_name, upstream_task_name
        )
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, NoPythonImports, BlankPrefectPreamble)]
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
impl TPrefectEncoding for ORCEncoding {
    fn get_prefect_decode_tasks(
        &self,
        _file_name: String,
        _task_name: String,
        _upstream_task_name: String,
    ) -> String { "".to_string() }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "spec")]
pub enum Encoding {
    CSVEncoding(CSVEncoding),
    ORCEncoding(ORCEncoding),
}
