#![allow(non_snake_case)]
use crate::asset::derived_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::supervised_model::*;
use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::encoding::*;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Asset {
    #[constrainable]
    DerivedAsset(DerivedAsset),
    #[constrainable]
    StaticDataTable(StaticDataTable),
    #[constrainable]
    SupervisedModel(SupervisedModel),
}

pub trait TAsset {
    fn get_name(&self) -> String;
    fn get_schema(&self) -> DataSchema;
    fn get_storage_setup(&self) -> StorageSetup;
    fn get_template_name(&self) -> String {
        self.get_schema().get_datum_template_name().unwrap()
    }
}

impl Asset {
    pub fn get_type(&self) -> String {
        match self {
            Asset::StaticDataTable(_) => "StaticDataTable",
            Asset::SupervisedModel(_) => "SupervisedModel",
            Asset::DerivedAsset(_) => "DerivedAsset",
        }
        .to_string()
    }
    pub fn get_name(&self) -> String {
        match self {
            Asset::StaticDataTable(x) => x.name.clone(),
            Asset::SupervisedModel(x) => x.name.clone(),
            Asset::DerivedAsset(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> DataSchema {
        match self {
            Asset::StaticDataTable(x) => x.schema.clone(),
            Asset::SupervisedModel(x) => x.schema.clone(),
            Asset::DerivedAsset(x) => x.schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> StorageSetup {
        match self {
            Asset::StaticDataTable(x) => x.setup.clone(),
            Asset::SupervisedModel(x) => x.setup.clone(),
            Asset::DerivedAsset(x) => x.setup.clone(),
        }
    }
}
impl InnerAsset {
    pub fn get_name(&self) -> String {
        match self {
            InnerAsset::StaticDataTable(x) => x.name.clone(),
            InnerAsset::SupervisedModel(x) => x.name.clone(),
            InnerAsset::DerivedAsset(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> InnerDataSchema {
        match self {
            InnerAsset::StaticDataTable(x) => x.schema.clone(),
            InnerAsset::SupervisedModel(x) => x.schema.clone(),
            InnerAsset::DerivedAsset(x) => x.schema.clone(),
        }
    }
    pub fn replicate_to_local(
        &self,
        t: InnerStorage,
        tmp_dir: String,
        tmp_encoding: InnerEncoding,
    ) -> Self {
        match self {
            InnerAsset::StaticDataTable(x) => {
                InnerAsset::StaticDataTable(x.replicate_to_local(t, tmp_dir, tmp_encoding))
            }
            InnerAsset::SupervisedModel(x) => {
                InnerAsset::SupervisedModel(x.replicate_to_local(t, tmp_dir, tmp_encoding))
            }
            InnerAsset::DerivedAsset(_) => panic!(
                "DerivedAssets are already stored locally, they cannot be replicated to local"
            ),
        }
    }
}
