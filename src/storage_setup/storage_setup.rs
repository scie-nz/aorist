#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::storage::*;
use crate::storage_setup::computed_from_local_data::*;
use crate::storage_setup::remote_import_storage_setup::*;
use crate::storage_setup::remote_storage_setup::*;
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
    #[constrainable]
    RemoteStorageSetup(RemoteStorageSetup),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> Vec<Storage> {
        match self {
            Self::RemoteStorageSetup(_) => vec![],
            Self::RemoteImportStorageSetup(s) => s.local.clone(),
            Self::ComputedFromLocalData(c) => vec![c.target.clone()],
        }
    }
    pub fn get_tmp_dir(&self) -> String {
        match self {
            Self::RemoteImportStorageSetup(s) => s.tmp_dir.clone(),
            Self::RemoteStorageSetup(_) => panic!("RemoteStorageSetup has no tmp_dir"),
            Self::ComputedFromLocalData(c) => c.tmp_dir.clone(),
        }
    }
}

impl InnerStorageSetup {
    pub fn replicate_to_local(&self, t: InnerStorage, tmp_dir: String) -> Self {
        match self {
            Self::RemoteStorageSetup(x) => Self::RemoteImportStorageSetup(x.replicate_to_local(t, tmp_dir)),
            _ => panic!("Only assets with RemoteStorageSetup can be replicated"),
        }
    }
}
