#![allow(non_snake_case)]
use crate::asset::derived_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::supervised_model::*;
use crate::concept::{AoristConcept, Concept};
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
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

impl Asset {
    pub fn get_name(&self) -> String {
        match self {
            Asset::StaticDataTable(x) => x.name.clone(),
            Asset::SupervisedModel(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> DataSchema {
        match self {
            Asset::StaticDataTable(x) => x.schema.clone(),
            Asset::SupervisedModel(x) => x.schema.clone(),
        }
    }
    pub fn get_storage_setup(&self) -> StorageSetup {
        match self {
            Asset::StaticDataTable(x) => x.setup.clone(),
            Asset::SupervisedModel(x) => x.setup.clone(),
        }
    }
}
impl InnerAsset {
    pub fn get_name(&self) -> String {
        match self {
            InnerAsset::StaticDataTable(x) => x.name.clone(),
            InnerAsset::SupervisedModel(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> InnerDataSchema {
        match self {
            InnerAsset::StaticDataTable(x) => x.schema.clone(),
            InnerAsset::SupervisedModel(x) => x.schema.clone(),
        }
    }
}
