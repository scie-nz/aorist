use crate::asset::geospatial_asset::*;
use crate::asset::graph_asset::*;
use crate::asset::language_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::vision_asset::*;
use crate::encoding::Encoding;
#[cfg(feature = "python")]
use crate::encoding::PyEncoding;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::asset_enum;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait TAsset {
    fn get_name(&self) -> AString;
    fn get_schema(&self) -> AoristRef<DataSchema>;
    fn get_storage_setup(&self) -> AoristRef<StorageSetup>;
    fn get_template_name(&self) -> AString {
        self.get_schema()
            .0
            .read()
            .get_datum_template_name()
            .unwrap()
    }
}

asset_enum! {
    name: Asset
    concrete_variants:
    - StaticDataTable
    enum_variants:
    - GeospatialAsset
    - LanguageAsset
    - GraphAsset
    - VisionAsset
}

impl Asset {
    pub fn persist_local(&self, persistent: AoristRef<Storage>) -> Self {
        let mut cloned = self.clone();
        let storage_setup = cloned.get_storage_setup();
        let new_setup = match *storage_setup.0.read() {
            StorageSetup::LocalStorageSetup(_) => AoristRef(RArc::new(RRwLock::new(
                cloned
                    .get_storage_setup()
                    .0
                    .read()
                    .persist_local(persistent),
            ))),
            _ => cloned.get_storage_setup(),
        };
        cloned.set_storage_setup(new_setup);
        cloned
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyAsset {
    pub fn persist_local(&self, storage: PyStorage) -> PyResult<Self> {
        Ok(PyAsset {
            inner: AoristRef(RArc::new(RRwLock::new(
                self.inner
                    .0
                    .read()
                    .persist_local(storage.inner.clone()),
            ))),
        })
    }
    pub fn replicate_to_local(
        &self,
        storage: PyStorage,
        tmp_dir: AString,
        tmp_encoding: PyEncoding,
    ) -> PyResult<Self> {
        Ok(PyAsset {
            inner: AoristRef(RArc::new(RRwLock::new(
                self.inner
                    .0
                    .read()
                    .replicate_to_local(
                        storage.inner.clone(),
                        tmp_dir.clone(),
                        tmp_encoding.inner.clone(),
                    )
                    .unwrap(),
            ))),
        })
    }
}
