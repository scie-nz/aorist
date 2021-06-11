#![allow(non_snake_case)]
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, ConceptEnum};
use crate::encoding::*;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct StaticDataTable {
    pub name: String,
    #[constrainable]
    pub setup: StorageSetup,
    #[constrainable]
    pub schema: DataSchema,
}

impl TAsset for StaticDataTable {
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

impl StaticDataTable {
    pub fn replicate_to_local(&self, t: Storage, tmp_dir: String, tmp_encoding: Encoding) -> Self {
        Self {
            name: self.name.clone(),
            setup: self.setup.replicate_to_local(t, tmp_dir, tmp_encoding),
            schema: self.schema.clone(),
            tag: self.tag.clone(),
            uuid: None,
        }
    }
}
