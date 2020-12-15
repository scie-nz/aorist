#![allow(non_snake_case)]

use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::endpoints::EndpointConfig;
use crate::hive::THiveTableCreationTagMutator;
use crate::prefect::{TObjectWithPrefectCodeGen, TPrefectStorage};
use crate::python::TObjectWithPythonCodeGen;
use crate::schema::DataSchema;
use crate::storage::hive_table_storage::HiveTableStorage;
use crate::storage::remote_website_storage::RemoteStorage;
use crate::template::DatumTemplate;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum Storage {
    #[constrainable]
    RemoteStorage(RemoteStorage),
    #[constrainable]
    HiveTableStorage(HiveTableStorage),
}
impl Storage {
    pub fn is_hive_storage(&self) -> bool {
        match self {
            Storage::RemoteStorage { .. } => false,
            Storage::HiveTableStorage { .. } => true,
        }
    }
    pub fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
        endpoints: &EndpointConfig,
    ) -> Result<(), String> {
        match self {
            Storage::RemoteStorage(_) => {
                Err("Cannot create Hive table for remote location".to_string())
            }
            Storage::HiveTableStorage(x) => x.populate_table_creation_tags(tags, endpoints),
        }
    }
}
