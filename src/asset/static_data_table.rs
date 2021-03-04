#![allow(non_snake_case)]
use crate::asset::asset::TAsset;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::schema::*;
use crate::storage::*;
use crate::storage_setup::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
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

impl InnerStaticDataTable {
    pub fn replicate_to_local(&self, t: InnerStorage, tmp_dir: String) -> Self {
        Self {
            name: self.name.clone(),
            setup: self.setup.replicate_to_local(t, tmp_dir),
            schema: self.schema.clone(),
            tag: self.tag.clone(),
        }
    }
}
