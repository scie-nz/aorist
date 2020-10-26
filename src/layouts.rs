#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleFileLayout {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StorageLayout {
    SingleFileLayout(SingleFileLayout),
}
