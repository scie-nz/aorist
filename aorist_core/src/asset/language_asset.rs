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

#[aorist]
pub enum LanguageAsset {
    #[constrainable]
    FasttextEmbedding(AoristRef<FasttextEmbedding>),
    #[constrainable]
    NamedEntities(AoristRef<NamedEntities>),
    #[constrainable]
    TextCorpus(AoristRef<TextCorpus>),
}
impl LanguageAsset {
    pub fn set_storage_setup(&mut self, setup: AoristRef<StorageSetup>) {
        match self {
            Self::FasttextEmbedding(x) => x.0.write().unwrap().set_storage_setup(setup),
            Self::NamedEntities(x) => x.0.write().unwrap().set_storage_setup(setup),
            Self::TextCorpus(x) => x.0.write().unwrap().set_storage_setup(setup),
        }
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyLanguageAsset {
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

impl LanguageAsset {
    pub fn get_type(&self) -> String {
        match self {
            LanguageAsset::NamedEntities(_) => "LanguageAsset",
            LanguageAsset::FasttextEmbedding(_) => "FasttextEmbedding",
            LanguageAsset::TextCorpus(_) => "TextCorpus",
        }
        .to_string()
    }
    pub fn get_name(&self) -> String {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().name.clone(),
            LanguageAsset::TextCorpus(x) => x.0.read().unwrap().name.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().name.clone(),
        }
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().schema.clone(),
            LanguageAsset::TextCorpus(x) => x.0.read().unwrap().schema.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        match self {
            LanguageAsset::FasttextEmbedding(x) => x.0.read().unwrap().setup.clone(),
            LanguageAsset::TextCorpus(x) => x.0.read().unwrap().setup.clone(),
            LanguageAsset::NamedEntities(x) => x.0.read().unwrap().setup.clone(),
        }
    }
    pub fn get_source_assets(&self) -> Vec<Asset> {
        let source_schema = match &*self.get_schema().0.read().unwrap() {
            DataSchema::LanguageAssetSchema(x) => x.0.read().unwrap().get_source_schema(),
            _ => panic!("schema must be LanguageAssetSchema"),
        };
        let sources = source_schema.0.read().unwrap().get_sources();
        sources
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
