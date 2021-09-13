#![allow(non_snake_case)]
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::text_corpus_schema::*;
use crate::schema::spacy_named_entity_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum NamedEntitySchema {
    #[constrainable]
    SpaCyNamedEntitySchema(AoristRef<SpaCyNamedEntitySchema>),
}

impl NamedEntitySchema {
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match self {
            NamedEntitySchema::SpaCyNamedEntitySchema(x) => {
                x.0.read().unwrap().get_source_schema()
            }
        }
    }
    pub fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        match self {
            Self::SpaCyNamedEntitySchema(x) => x.0.read().unwrap().get_attributes(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            NamedEntitySchema::SpaCyNamedEntitySchema(x) => {
                x.0.read().unwrap().get_datum_template()
            }
        }
    }
}
