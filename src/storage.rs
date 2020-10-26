#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use crate::locations::RemoteWebsiteLocation;
use crate::layouts::StorageLayout;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoteWebsiteStorage {
    location: RemoteWebsiteLocation,
    layout: StorageLayout,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Storage {
    RemoteWebsiteStorage(RemoteWebsiteStorage),
}

