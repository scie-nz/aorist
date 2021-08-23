#![allow(non_snake_case)]
use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::schema::*;
use crate::storage::*;
use crate::encoding::*;
use crate::storage_setup::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist]
pub struct RasterAsset {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
}
impl TAsset for RasterAsset {
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

impl RasterAsset {
    pub fn replicate_to_local(
        &self,
        t: AoristRef<Storage>,
        tmp_dir: String,
        tmp_encoding: AoristRef<Encoding>,
    ) -> Self {
        Self {
            name: self.name.clone(),
            comment: self.comment.clone(),
            setup: AoristRef(Arc::new(RwLock::new(
                self.setup
                    .0
                    .read()
                    .unwrap()
                    .replicate_to_local(t, tmp_dir, tmp_encoding),
            ))),
            schema: self.schema.clone(),
            tag: self.tag.clone(),
            uuid: None,
        }
    }
}