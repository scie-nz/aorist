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
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[aorist]
pub enum LanguageAsset {
    #[constrainable]
    FasttextEmbedding(AoristRef<FasttextEmbedding>),
    #[constrainable]
    NamedEntities(AoristRef<NamedEntities>),
}

#[cfg(feature = "python")]
#[pymethods]
impl PyLanguageAsset {
    #[getter]
    pub fn get_source_assets(&self) -> Vec<PyAsset> {
        self.inner.0.read().unwrap().get_source_assets().iter().map(|x| PyAsset{ inner: x.clone() }).collect()
    }
    #[getter]
    pub fn get_storage_setup(&self) -> PyStorageSetup {
        PyStorageSetup { inner: self.inner.0.read().unwrap().get_storage_setup().clone() }
    }
}

impl LanguageAsset {
    pub fn get_source_assets(&self) -> Vec<AoristRef<Asset>> {
        match self {
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().get_source_assets(),
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().get_source_assets(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            LanguageAsset::NamedEntities(_) => "LanguageAsset",
            LanguageAsset::FasttextEmbedding(_) => "FasttextEmbedding",
        }
        .to_string()
    }
    pub fn get_name(&self) -> String {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().name.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().name.clone(),
        }
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().schema.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().setup.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().setup.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        _t: AoristRef<Storage>,
        _tmp_dir: String,
        _tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        panic!("Cannot replicate to local");
        // TODO: this should be implemented via a macro
        /*
        match self {
            LanguageAsset::FasttextEmbedding(x) => LanguageAsset::FasttextEmbedding(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
            LanguageAsset::NamedEntities(x) => LanguageAsset::NamedEntities(AoristRef(Arc::new(RwLock::new(
                x.0.read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            )))),
        }*/
    }
}
