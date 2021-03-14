#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::encoding::*;
use crate::storage::hive_table_storage::*;
use crate::storage::local_file_storage::*;
use crate::storage::remote_storage::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Storage {
    #[constrainable]
    RemoteStorage(RemoteStorage),
    #[constrainable]
    HiveTableStorage(HiveTableStorage),
    #[constrainable]
    LocalFileStorage(LocalFileStorage),
}

impl Storage {
    pub fn get_encoding(&self) -> Encoding {
        match &self {
            Self::RemoteStorage(x) => x.encoding.clone(),
            Self::HiveTableStorage(x) => x.encoding.clone(),
            Self::LocalFileStorage(x) => x.encoding.clone(),
        }
    }
}
