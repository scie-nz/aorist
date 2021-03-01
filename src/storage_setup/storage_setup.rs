#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::storage::*;
use crate::storage_setup::computed_from_local_data::*;
use crate::storage_setup::remote_import_storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum StorageSetup {
    #[constrainable]
    RemoteImportStorageSetup(RemoteImportStorageSetup),
    #[constrainable]
    ComputedFromLocalData(ComputedFromLocalData),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> Vec<Storage> {
        match self {
            Self::RemoteImportStorageSetup(s) => s.local.clone(),
            Self::ComputedFromLocalData(c) => vec![c.target.clone()],
        }
    }
    pub fn get_tmp_dir(&self) -> String {
        match self {
            Self::RemoteImportStorageSetup(s) => s.tmp_dir.clone(),
            Self::ComputedFromLocalData(c) => c.tmp_dir.clone(),
        }
    }
}
