use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use crate::encoding::*;
use crate::storage::*;
use crate::storage_setup::computed_from_local_data::*;
use crate::storage_setup::local_storage_setup::*;
use crate::storage_setup::remote_storage_setup::*;
use crate::storage_setup::replication_storage_setup::*;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum StorageSetup {
    #[constrainable]
    ReplicationStorageSetup(ReplicationStorageSetup),
    #[constrainable]
    ComputedFromLocalData(ComputedFromLocalData),
    #[constrainable]
    RemoteStorageSetup(RemoteStorageSetup),
    #[constrainable]
    LocalStorageSetup(LocalStorageSetup),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> Vec<Storage> {
        match self {
            Self::RemoteStorageSetup(_) => vec![],
            Self::ReplicationStorageSetup(s) => s.targets.clone(),
            Self::ComputedFromLocalData(c) => vec![c.target.clone()],
            Self::LocalStorageSetup(l) => vec![l.local.clone()],
        }
    }
    pub fn get_tmp_dir(&self) -> String {
        match self {
            Self::ReplicationStorageSetup(s) => s.tmp_dir.clone(),
            Self::RemoteStorageSetup(_) => panic!("RemoteStorageSetup has no tmp_dir"),
            Self::ComputedFromLocalData(c) => c.tmp_dir.clone(),
            Self::LocalStorageSetup(l) => l.tmp_dir.clone(),
        }
    }
    pub fn replicate_to_local(&self, t: Storage, tmp_dir: String, tmp_encoding: Encoding) -> Self {
        match self {
            Self::RemoteStorageSetup(x) => {
                Self::ReplicationStorageSetup(x.replicate_to_local(t, tmp_dir, tmp_encoding))
            }
            _ => panic!("Only assets with RemoteStorageSetup can be replicated"),
        }
    }
}
