#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use crate::locations::{RemoteWebsiteLocation, HiveLocation};
use crate::layouts::{StorageLayout, HiveStorageLayout};
use crate::encoding::Encoding;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoteWebsiteStorage {
    location: RemoteWebsiteLocation,
    layout: StorageLayout,
    encoding: Encoding,
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HiveTableStorage {
    location: HiveLocation,
    layout: HiveStorageLayout,
    encoding: Encoding,
}
impl HiveTableStorage {
    pub fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        self.location.populate_table_creation_tags(tags)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Storage {
    RemoteWebsiteStorage(RemoteWebsiteStorage),
    HiveTableStorage(HiveTableStorage),
}
impl Storage {
    pub fn is_hive_storage(&self) -> bool {
        match self {
            Storage::RemoteWebsiteStorage{..} => false,
            Storage::HiveTableStorage{..} => true,
        }
    }
    pub fn populate_table_creation_tags(
        &self,
        tags: &mut HashMap<String, String>,
    ) -> Result<(), String> {
        match self {
            Storage::RemoteWebsiteStorage(_) => Err(
                "Cannot create Hive table for remote location".to_string()
            ),
            Storage::HiveTableStorage(x) => x.populate_table_creation_tags(tags)
        }
    }
}
