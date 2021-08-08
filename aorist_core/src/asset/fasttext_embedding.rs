#![allow(non_snake_case)]
use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::schema::*;
use crate::storage_setup::*;
use crate::template::TDatumTemplate;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist]
pub struct FasttextEmbedding {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
    #[constrainable]
    pub source_assets: Vec<AoristRef<Asset>>,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
}
impl TDatumTemplate for FasttextEmbedding {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        vec![AoristRef(Arc::new(RwLock::new(Attribute {
            inner: AttributeOrTransform::Attribute(AttributeEnum::VectorEmbedding(
                VectorEmbedding {
                    name: self.name.clone(),
                    comment: self.comment.clone(),
                    nullable: false,
                },
            )),
            tag: None,
            uuid: None,
        })))]
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
