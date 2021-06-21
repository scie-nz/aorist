#![allow(non_snake_case)]
use crate::algorithms::*;
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct SupervisedModel {
    pub name: String,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
    #[constrainable]
    pub algorithm: AoristRef<RegressionAlgorithm>,
}

impl TAsset for SupervisedModel {
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
