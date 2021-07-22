#![allow(non_snake_case)]
use crate::attributes::*;
use crate::asset::*;
use crate::schema::*;
use crate::storage::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use aorist_attributes::*;

#[aorist]
pub struct FasttextEmbedding {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
    pub schema: AoristRef<FasttextEmbeddingSchema>, 
    #[constrainable]
    pub source_assets: Vec<AoristRef<Asset>>,
    #[constrainable]
    pub storage: AoristRef<Storage>,
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
