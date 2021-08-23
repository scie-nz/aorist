#![allow(non_snake_case)]
use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use std::sync::{Arc, RwLock};

#[aorist]
pub enum GeospatialAsset {
    #[constrainable]
    RasterAsset(AoristRef<RasterAsset>),
}

#[cfg(feature = "python")]
#[pymethods]
impl PyGeospatialAsset {
    #[getter]
    pub fn get_storage_setup(&self) -> PyStorageSetup {
        PyStorageSetup {
            inner: self.inner.0.read().unwrap().get_storage_setup().clone(),
        }
    }
    #[getter]
    pub fn get_schema(&self) -> PyDataSchema {
        PyDataSchema {
            inner: self.inner.0.read().unwrap().get_schema().clone(),
        }
    }
}

impl GeospatialAsset {
    pub fn get_type(&self) -> String {
        match self {
            GeospatialAsset::RasterAsset(_) => "RasterAsset",
        }
        .to_string()
    }
    pub fn get_name(&self) -> String {
        match self {
            GeospatialAsset::RasterAsset(x) => x.0.read().unwrap().name.clone(),
        }
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        match self {
            GeospatialAsset::RasterAsset(x) => x.0.read().unwrap().schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        match self {
            GeospatialAsset::RasterAsset(x) => x.0.read().unwrap().setup.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        match self {
            GeospatialAsset::RasterAsset(x) => GeospatialAsset::RasterAsset(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
        }
    }
}
