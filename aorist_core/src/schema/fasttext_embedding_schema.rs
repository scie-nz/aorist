use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConceptBase, AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::data_schema::DataSchema;
use crate::schema::derived_asset_schema::*;
use crate::schema::language_asset_schema::LanguageAssetSchema;
use crate::schema::text_corpus_schema::TextCorpusSchema;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: FasttextEmbeddingSchema,
    source: TextCorpus,
    attributes:
      token: KeyStringIdentifier("token", false),
      embedding: VectorEmbedding("embedding", false)
    fields:
      dim: usize
}

impl FasttextEmbeddingSchema {
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
