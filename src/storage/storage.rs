#![allow(non_snake_case)]

use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{
    TObjectWithPrefectCodeGen,
    TPrefectStorage,
};
use crate::python::TObjectWithPythonCodeGen;
use crate::storage::remote_website_storage::RemoteWebsiteStorage;
use crate::storage::hive_table_storage::HiveTableStorage;
use crate::schema::DataSchema;
use crate::templates::DatumTemplate;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
