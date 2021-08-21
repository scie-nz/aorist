#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::text_corpus_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum NamedEntitySchema {
    #[constrainable]
    SpacyNamedEntitySchema(AoristRef<SpacyNamedEntitySchema>),
}

impl NamedEntitySchema {
    pub fn source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match self {
            NamedEntitySchema::SpacyNamedEntitySchema(x) => {
                x.0.read().unwrap().source_schema.clone()
            }
        }
    }
}

#[aorist]
pub struct SpacyNamedEntitySchema {
    pub spacy_model_name: String,
    pub text_attribute_name: String,
    #[constrainable]
    pub source_schema: AoristRef<TextCorpusSchema>,
}
