use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::data_schema::DataSchema;
use crate::schema::derived_asset_schema::*;
use crate::schema::language_asset_schema::LanguageAssetSchema;
use crate::schema::text_corpus_schema::TextCorpusSchema;
use crate::template::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: SpaCyNamedEntitySchema,
    source: TextCorpus,
    attributes:
      document_id: StringIdentifier("document id", false),
      named_entity: FreeText("Named Entity Text", false),
      start: CharacterPosition("start of named entity location", false),
      end: CharacterPosition("end of named entity location", false),
      label: Factor("named entity label", false),
      description: Factor("named entity description", false)
    fields:
      spacy_model_name: String
}

impl SpaCyNamedEntitySchema {
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match &*self.get_source().0.read().get_schema().0.read() {
            DataSchema::LanguageAssetSchema(l) => match &*l.0.read() {
                LanguageAssetSchema::TextCorpusSchema(x) => x.clone(),
                _ => panic!("Source schema must be TextCorpusSchema"),
            },
            _ => panic!("Source schema must be LanguageAssetSchema"),
        }
    }
}
