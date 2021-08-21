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
pub struct NamedEntities {
    pub name: String,
    pub comment: Option<String>,
    #[constrainable]
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
            AoristRef(Arc::new(RwLock::new(Attribute {
                inner: AttributeOrTransform::Attribute(AttributeEnum::FreeText(FreeText {
                    name: "text".to_string(),
                    comment: Some("Named Entity Text".to_string()),
                    nullable: false,
                })),
                tag: None,
                uuid: None,
            }))),
            AoristRef(Arc::new(RwLock::new(Attribute {
                inner: AttributeOrTransform::Attribute(AttributeEnum::CharacterPosition(
                    CharacterPosition {
                        name: "start".to_string(),
                        comment: Some("start of named entity location".to_string()),
                        nullable: false,
                    },
                )),
                tag: None,
                uuid: None,
            }))),
            AoristRef(Arc::new(RwLock::new(Attribute {
                inner: AttributeOrTransform::Attribute(AttributeEnum::CharacterPosition(
                    CharacterPosition {
                        name: "end".to_string(),
                        comment: Some("end of named entity location".to_string()),
                        nullable: false,
                    },
                )),
                tag: None,
                uuid: None,
            }))),
            AoristRef(Arc::new(RwLock::new(Attribute {
                inner: AttributeOrTransform::Attribute(AttributeEnum::Factor(Factor {
                    name: "label".to_string(),
                    comment: Some("named entity label".to_string()),
                    nullable: false,
                })),
                tag: None,
                uuid: None,
            }))),
            AoristRef(Arc::new(RwLock::new(Attribute {
                inner: AttributeOrTransform::Attribute(AttributeEnum::Factor(Factor {
                    name: "description".to_string(),
                    comment: Some("Named Entity Description".to_string()),
                    nullable: false,
                })),
                tag: None,
                uuid: None,
            }))),
        ]
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
