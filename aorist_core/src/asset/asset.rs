#![allow(non_snake_case)]
use crate::asset::derived_asset::*;
use crate::asset::geospatial_asset::*;
use crate::asset::language_asset::*;
use crate::asset::static_data_table::*;
use crate::asset::supervised_model::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_primitives::asset_enum;
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

asset_enum! {
    name: Asset
    concrete_variants:
    - StaticDataTable
    - SupervisedModel
    enum_variants:
    - GeospatialAsset
    - LanguageAsset
}

impl Asset {
    pub fn persist_local(
        &self,
        persistent: AoristRef<Storage>,
    ) -> Self {
        let mut cloned = self.clone();
        let storage_setup = cloned.get_storage_setup();
        let new_setup = match *storage_setup.0.read().unwrap() {
            StorageSetup::LocalStorageSetup(_) => 
                AoristRef(Arc::new(RwLock::new(
                    cloned.get_storage_setup().0.read().unwrap()
                      .persist_local(persistent)
                ))),
            _ => cloned.get_storage_setup(),
        };
        cloned.set_storage_setup(new_setup);
        cloned
    }
}
