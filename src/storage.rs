#![allow(non_snake_case)]

use crate::data_setup::EndpointConfig;
use crate::encoding::Encoding;
use crate::hive::THiveTableCreationTagMutator;
use crate::layouts::{HiveStorageLayout, StorageLayout};
use crate::locations::{HiveLocation, RemoteWebsiteLocation};
use crate::prefect::{
    TObjectWithPrefectCodeGen, TPrefectEncoding, TPrefectHiveLocation, TPrefectLocation,
    TPrefectStorage,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.location.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectStorage for RemoteWebsiteStorage {
    fn get_prefect_dag(&self, _schema: &DataSchema) -> Result<String, String> {
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
    fn get_prefect_ingest_dag(
        &self,
        _path: String,
        _filename: String,
        _schema: &DataSchema,
        _templates: &HashMap<String, DatumTemplate>,
        _task_name: String,
        _upstream_task_name: String,
        _endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        Err("Ingest dag not implemented".to_string())
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
        endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        self.encoding
            .populate_table_creation_tags(tags, endpoints)?;
        self.location.populate_table_creation_tags(tags, endpoints)
    }
}
impl TObjectWithPythonCodeGen for HiveTableStorage {
    fn get_python_imports(&self, preamble: &mut HashMap<String, String>) {
        self.location.get_python_imports(preamble);
    }
}
impl TObjectWithPrefectCodeGen for HiveTableStorage {
    fn get_prefect_preamble(
        &self,
        preamble: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) {
        self.location.get_prefect_preamble(preamble, endpoints);
    }
}
impl TPrefectStorage for HiveTableStorage {
    fn get_prefect_dag(&self, _schema: &DataSchema) -> Result<String, String> {
        Err("Ingest dag not implemented".to_string())
    }
    fn get_prefect_ingest_dag(
        &self,
        local_path: String,
        filename: String,
        schema: &DataSchema,
        templates: &HashMap<String, DatumTemplate>,
        task_name: String,
        upstream_task_name: String,
        endpoints: &EndpointConfig,
    ) -> Result<String, String> {
        Ok(format!(
            "{}\n{}",
            self.encoding.get_prefect_encode_tasks(
                format!("{}/{}", &local_path, &filename).to_string(),
                format!("{}.encoded", filename),
                format!("{}_encode", task_name).to_string(),
                upstream_task_name,
                schema,
                templates,
            ),
            self.location.get_prefect_upload_task(
                format!("{}.encoded", filename),
                local_path,
                format!("{}_upload", task_name).to_string(),
                format!("{}_encode", task_name).to_string(),
                endpoints,
            ),
        ))
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
        endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        match self {
            Storage::RemoteWebsiteStorage(_) => {
                Err("Cannot create Hive table for remote location".to_string())
            }
            Storage::HiveTableStorage(x) => x.populate_table_creation_tags(tags, endpoints),
        }
    }
}
