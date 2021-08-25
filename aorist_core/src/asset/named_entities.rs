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
use aorist_primitives::{AoristConcept, ConceptEnum, attribute};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist]
pub struct NamedEntities {
    pub name: String,
    pub comment: Option<String>,
    pub source_assets: Vec<AoristRef<Asset>>,
    #[constrainable]
    pub setup: AoristRef<StorageSetup>,
    #[constrainable]
    pub schema: AoristRef<DataSchema>,
}
impl NamedEntities {
    pub fn get_source_assets(&self) -> Vec<AoristRef<Asset>> {
        self.source_assets.clone()
    }
    pub fn get_schema(&self) -> AoristRef<DataSchema> {
        self.schema.clone()
    }
}
impl TDatumTemplate for NamedEntities {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        vec![
            attribute! { FreeText(
                "text".to_string(), 
                Some("Named Entity Text".to_string()), 
                false
            ) }, 
            attribute! { CharacterPosition(
                "start".to_string(), 
                Some("start of named entity location".to_string()),
                false
            ) }, 
            attribute! { CharacterPosition(
                "end".to_string(), 
                Some("end of named entity location".to_string()),
                false
            ) }, 
            attribute! { Factor(
                "label".to_string(), 
                Some("named entity label".to_string()),
                false
            ) }, 
            attribute! { FreeText(
                "description".to_string(), 
                Some("named entity description".to_string()),
                false
            ) }, 
        ]
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
