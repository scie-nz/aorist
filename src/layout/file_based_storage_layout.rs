#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileBasedStorageLayout {
    SingleFileLayout(SingleFileLayout),
}
