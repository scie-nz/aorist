#![allow(non_snake_case)]
use crate::asset::static_data_table::*;
use crate::concept::{AoristConcept, Concept};
use crate::schema::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum Asset {
    #[constrainable]
    StaticDataTable(StaticDataTable),
}

impl Asset {
    pub fn get_name(&self) -> String {
        match self {
            Asset::StaticDataTable(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> DataSchema {
        match self {
            Asset::StaticDataTable(x) => x.schema.clone(),
        }
    }
}
impl InnerAsset {
    pub fn get_name(&self) -> String {
        match self {
            InnerAsset::StaticDataTable(x) => x.name.clone(),
        }
    }
    pub fn get_schema(&self) -> InnerDataSchema {
        match self {
            InnerAsset::StaticDataTable(x) => x.schema.clone(),
        }
    }
}
