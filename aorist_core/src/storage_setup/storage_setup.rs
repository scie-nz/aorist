use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::storage::*;
use crate::storage_setup::computed_from_local_data::*;
use crate::storage_setup::local_storage_setup::*;
use crate::storage_setup::remote_storage_setup::*;
use crate::storage_setup::replication_storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist]
pub enum StorageSetup {
    #[constrainable]
    ReplicationStorageSetup(AoristRef<ReplicationStorageSetup>),
    #[constrainable]
    ComputedFromLocalData(AoristRef<ComputedFromLocalData>),
    #[constrainable]
    RemoteStorageSetup(AoristRef<RemoteStorageSetup>),
    #[constrainable]
    LocalStorageSetup(AoristRef<LocalStorageSetup>),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> Vec<AoristRef<Storage>> {
        match self {
            Self::RemoteStorageSetup(_) => vec![],
            Self::ReplicationStorageSetup(x) => x.0.read().unwrap().targets.clone(),
            Self::ComputedFromLocalData(x) => vec![x.0.read().unwrap().target.clone()],
            Self::LocalStorageSetup(x) => vec![x.0.read().unwrap().local.clone()],
        }
    }
    pub fn get_tmp_dir(&self) -> String {
        match self {
            Self::ReplicationStorageSetup(x) => x.0.read().unwrap().tmp_dir.clone(),
            Self::RemoteStorageSetup(_) => panic!("RemoteStorageSetup has no tmp_dir"),
            Self::ComputedFromLocalData(x) => x.0.read().unwrap().tmp_dir.clone(),
            Self::LocalStorageSetup(x) => x.0.read().unwrap().tmp_dir.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        match self {
            Self::RemoteStorageSetup(x) => {
                Self::ReplicationStorageSetup(AoristRef(Arc::new(RwLock::new(
                    x.0.read()
                        .unwrap()
                        .replicate_to_local(t, tmp_dir, tmp_encoding),
                ))))
            }
            _ => panic!("Only assets with RemoteStorageSetup can be replicated"),
        }
    }
}
