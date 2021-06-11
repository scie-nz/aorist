#![allow(non_snake_case)]
use crate::algorithms::*;
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, ConceptEnum};
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct SupervisedModel {
    pub name: String,
    #[constrainable]
    pub setup: StorageSetup,
    #[constrainable]
    pub schema: DataSchema,
    #[constrainable]
    pub algorithm: RegressionAlgorithm,
}

impl TAsset for SupervisedModel {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_schema(&self) -> DataSchema {
        self.schema.clone()
    }
    fn get_storage_setup(&self) -> StorageSetup {
        self.setup.clone()
    }
}
