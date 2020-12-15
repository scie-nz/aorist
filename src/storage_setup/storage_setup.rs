#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectStorageSetup};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage::Storage;
use crate::storage_setup::remote_import_storage_setup::RemoteImportStorageSetup;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum StorageSetup {
    #[constrainable]
    RemoteImportStorageSetup(RemoteImportStorageSetup),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> &Vec<Storage> {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => x.get_local_storage(),
        }
    }
    pub fn get_presto_schemas(
        &self,
        name: &String,
        columnSchema: String,
        endpoints: &EndpointConfig,
    ) -> String {
        match self {
            StorageSetup::RemoteImportStorageSetup(x) => {
                x.get_presto_schemas(name, columnSchema, endpoints)
            }
        }
    }
}
