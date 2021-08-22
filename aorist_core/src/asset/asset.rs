#![allow(non_snake_case)]
use crate::asset::derived_asset::*;
use crate::asset::language_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::supervised_model::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist]
pub enum Asset {
    #[constrainable]
    DerivedAsset(AoristRef<DerivedAsset>),
    #[constrainable]
    LanguageAsset(AoristRef<LanguageAsset>),
    #[constrainable]
    StaticDataTable(AoristRef<StaticDataTable>),
    #[constrainable]
    SupervisedModel(AoristRef<SupervisedModel>),
}

pub trait TAsset {
    fn get_name(&self) -> String;
    fn get_schema(&self) -> AoristRef<DataSchema>;
    fn get_storage_setup(&self) -> AoristRef<StorageSetup>;
    fn get_template_name(&self) -> String {
        self.get_schema()
            .0
            .read()
            .unwrap()
            .get_datum_template_name()
            .unwrap()
    }
}

impl Asset {
    pub fn get_type(&self) -> String {
        match self {
            Asset::DerivedAsset(_) => "DerivedAsset".into(),
            Asset::StaticDataTable(_) => "StaticDataTable".into(),
            Asset::SupervisedModel(_) => "SupervisedModel".into(),
            Asset::LanguageAsset(x) => x.0.read().unwrap().get_type(),
        }
    }
    pub fn get_name(&self) -> String {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().name.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().name.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().name.clone(),
            Asset::LanguageAsset(x) => x.0.read().unwrap().get_name(),
        }
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().schema.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().schema.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().schema.clone(),
            Asset::LanguageAsset(x) => x.0.read().unwrap().get_schema(),
        }
    }
    pub fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        match self {
            Asset::StaticDataTable(x) => x.0.read().unwrap().setup.clone(),
            Asset::SupervisedModel(x) => x.0.read().unwrap().setup.clone(),
            Asset::DerivedAsset(x) => x.0.read().unwrap().setup.clone(),
            Asset::LanguageAsset(x) => x.0.read().unwrap().get_storage_setup(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        match self {
            Asset::StaticDataTable(x) => Asset::StaticDataTable(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
            Asset::SupervisedModel(x) => Asset::SupervisedModel(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
            Asset::LanguageAsset(x) => Asset::LanguageAsset(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
            _ => self.clone(),
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyAsset {
    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Ok(self.inner.0.read().unwrap().get_name())
    }
    #[getter]
    pub fn schema(&self) -> PyResult<PyDataSchema> {
        Ok(PyDataSchema {
            inner: self.inner.0.read().unwrap().get_schema().clone(),
        })
    }
    #[getter]
    pub fn storage_setup(&self) -> PyResult<PyStorageSetup> {
        Ok(PyStorageSetup {
            inner: self.inner.0.read().unwrap().get_storage_setup().clone(),
        })
    }
}
