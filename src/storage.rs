#![allow(non_snake_case)]

use crate::encoding::Encoding;
use crate::hive::THiveTableCreationTagMutator;
use crate::layouts::{HiveStorageLayout, StorageLayout};
use crate::locations::{HiveLocation, RemoteWebsiteLocation};
use crate::python::TObjectWithPythonCodeGen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use enum_dispatch::enum_dispatch;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectLocation, TObjectWithPrefectDAGCodeGen, TPrefectEncoding};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoteWebsiteStorage {
    location: RemoteWebsiteLocation,
    layout: StorageLayout,
    encoding: Encoding,
}
impl TObjectWithPythonCodeGen for RemoteWebsiteStorage {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_python_imports(preamble);
        self.encoding.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for RemoteWebsiteStorage {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_prefect_preamble(preamble);
    }
}
impl TObjectWithPrefectDAGCodeGen for RemoteWebsiteStorage {
    fn get_prefect_dag(&self) -> Result<String, String> {
        Ok(format!(
            "{}\n{}",
            self.location.get_prefect_download_task(
                "download_remote".to_string(),
                "/tmp/materialized_file".to_string(),
            ),
            self.encoding.get_prefect_decode_tasks(
                "/tmp/materialized_file".to_string(),
                "decode_file".to_string(),
                "download_remote".to_string(),
            )
        ))
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HiveTableStorage {
    location: HiveLocation,
    layout: HiveStorageLayout,
    encoding: Encoding,
}
impl THiveTableCreationTagMutator for HiveTableStorage {
    fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        self.encoding.populate_table_creation_tags(tags)?;
        self.location.populate_table_creation_tags(tags)
    }
}
impl TObjectWithPythonCodeGen for HiveTableStorage {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for HiveTableStorage {
    fn get_prefect_preamble(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_prefect_preamble(preamble);
    }
}
impl TObjectWithPrefectDAGCodeGen for HiveTableStorage {
    fn get_prefect_dag(&self) -> Result<String, String> {
        Err("No Prefect DAG has been implemented".to_string())
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Storage {
    RemoteWebsiteStorage(RemoteWebsiteStorage),
    HiveTableStorage(HiveTableStorage),
}
impl Storage {
    pub fn is_hive_storage(&self) -> bool {
        match self {
            Storage::RemoteWebsiteStorage { .. } => false,
            Storage::HiveTableStorage { .. } => true,
        }
    }
    pub fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        match self {
            Storage::RemoteWebsiteStorage(_) => {
                Err("Cannot create Hive table for remote location".to_string())
            }
            Storage::HiveTableStorage(x) => x.populate_table_creation_tags(tags),
        }
    }
}
