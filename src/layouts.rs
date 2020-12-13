#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

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
pub struct DailyGranularity {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Granularity {
    DailyGranularity(DailyGranularity),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DynamicHiveTableLayout {
    granularity: DailyGranularity,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
    DynamicHiveTableLayout(DynamicHiveTableLayout),
}
