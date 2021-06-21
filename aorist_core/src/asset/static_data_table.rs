#![allow(non_snake_case)]
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use crate::encoding::*;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, RwLock};

#[aorist]
pub struct StaticDataTable {
    pub name: String,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
}

impl TAsset for StaticDataTable {
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

impl StaticDataTable {
    pub fn replicate_to_local(&self, t: AoristRef<Storage>, tmp_dir: String, tmp_encoding: AoristRef<Encoding>) -> Self {
        Self {
            name: self.name.clone(),
            setup: AoristRef(Arc::new(RwLock::new(self.setup.0.read().unwrap().replicate_to_local(t, tmp_dir, tmp_encoding)))),
            schema: self.schema.clone(),
            tag: self.tag.clone(),
            uuid: None,
        }
    }
}
