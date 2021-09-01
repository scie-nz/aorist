#![allow(non_snake_case)]
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct DerivedAsset {
    pub name: String,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
}

impl DerivedAsset {
    pub fn set_storage_setup(&mut self, setup: AoristRef<StorageSetup>) {
        self.setup = setup;
    }
}

impl TAsset for DerivedAsset {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_schema(&self) -> AoristRef<DataSchema> {
        self.schema.clone()
    }
    fn get_storage_setup(&self) -> AoristRef<StorageSetup> {
        self.setup.clone()
    }
}
