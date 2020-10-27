#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum StorageLayout {
    SingleFileLayout(SingleFileLayout),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StaticHiveTableLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
}
