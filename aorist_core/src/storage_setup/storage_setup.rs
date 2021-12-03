use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::storage::*;
use crate::storage_setup::computed_from_local_data::*;
use crate::storage_setup::local_storage_setup::*;
use crate::storage_setup::remote_storage_setup::*;
use crate::storage_setup::replication_storage_setup::*;
use crate::storage_setup::two_tier_storage_setup::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
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
    #[constrainable]
    TwoTierStorageSetup(AoristRef<TwoTierStorageSetup>),
}

impl StorageSetup {
    pub fn get_local_storage(&self) -> Vec<AoristRef<Storage>> {
        match self {
            Self::RemoteStorageSetup(_) => vec![],
            Self::ReplicationStorageSetup(x) => x.0.read().targets.clone(),
            Self::ComputedFromLocalData(x) => vec![x.0.read().target.clone()],
            Self::LocalStorageSetup(x) => vec![x.0.read().local.clone()],
            Self::TwoTierStorageSetup(x) => {
                vec![x.0.read().scratch.clone(), x.0.read().persistent.clone()]
            }
        }
    }
    pub fn get_tmp_dir(&self) -> String {
        match self {
            Self::ReplicationStorageSetup(x) => x.0.read().tmp_dir.clone(),
            Self::RemoteStorageSetup(x) => x.0.read().tmp_dir.as_ref().unwrap().clone(),
            Self::ComputedFromLocalData(x) => x.0.read().tmp_dir.clone(),
            Self::LocalStorageSetup(x) => x.0.read().tmp_dir.clone(),
            Self::TwoTierStorageSetup(x) => x.0.read().tmp_dir.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        match self {
            Self::RemoteStorageSetup(x) => Self::ReplicationStorageSetup(AoristRef(RArc::new(
                RRwLock::new(x.0.read().replicate_to_local(t, tmp_dir, tmp_encoding)),
            ))),
            _ => panic!("Only assets with RemoteStorageSetup can be replicated"),
        }
    }
    pub fn persist_local(&self, persistent: AoristRef<Storage>) -> Self {
        match self {
            Self::LocalStorageSetup(x) => Self::TwoTierStorageSetup(AoristRef(RArc::new(
                RRwLock::new(x.0.read().persist(persistent)),
            ))),
            _ => panic!("Only assets with LocalStorageSetup can be persisted"),
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyStorageSetup {
    #[getter]
    pub fn tmp_dir(&self) -> String {
        self.inner.0.read().get_tmp_dir()
    }
    #[getter]
    pub fn local(&self) -> Vec<PyStorage> {
        self.inner
            .0
            .read()
            .get_local_storage()
            .into_iter()
            .map(|x| PyStorage { inner: x })
            .collect()
    }
}
