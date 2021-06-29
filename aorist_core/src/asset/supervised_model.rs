#![allow(non_snake_case)]
use crate::algorithms::*;
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
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
impl SupervisedModel {
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        Self {
            name: self.name.clone(),
            setup: AoristRef(Arc::new(RwLock::new(
                self.setup
                    .0
                    .read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            ))),
            schema: self.schema.clone(),
            algorithm: self.algorithm.clone(),
            tag: self.tag.clone(),
            uuid: None,
        }
    }
}
