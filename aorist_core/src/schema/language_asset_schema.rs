use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::fasttext_embedding_schema::*;
use crate::schema::named_entity_schema::*;
use crate::schema::text_corpus_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum LanguageAssetSchema {
    #[constrainable]
    FasttextEmbeddingSchema(AoristRef<FasttextEmbeddingSchema>),
    #[constrainable]
    NamedEntitySchema(AoristRef<NamedEntitySchema>),
    #[constrainable]
    TextCorpusSchema(AoristRef<TextCorpusSchema>),
}

impl LanguageAssetSchema {
    pub fn get_attributes(&self) -> AVec<AoristRef<Attribute>> {
        match self {
            Self::FasttextEmbeddingSchema(x) => x.0.read().get_attributes(),
            Self::NamedEntitySchema(x) => x.0.read().get_attributes(),
            Self::TextCorpusSchema(x) => x.0.read().get_attributes(),
        }
    }
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match self {
            LanguageAssetSchema::FasttextEmbeddingSchema(x) => x.0.read().get_source_schema(),
            LanguageAssetSchema::NamedEntitySchema(x) => x.0.read().get_source_schema(),
            LanguageAssetSchema::TextCorpusSchema(x) => x.clone(),
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            LanguageAssetSchema::FasttextEmbeddingSchema(x) => x.0.read().get_datum_template(),
            LanguageAssetSchema::NamedEntitySchema(x) => x.0.read().get_datum_template(),
            LanguageAssetSchema::TextCorpusSchema(x) => x.0.read().get_datum_template(),
        }
    }
    pub fn get_text_attribute_name(&self) -> AString {
        self.get_source_schema()
            .0
            .read()
            .text_attribute_name
            .clone()
    }
    pub fn get_datum_template_name(&self) -> AString {
        self.get_datum_template().0.read().get_name()
    }
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.get_source_schema()
            .0
            .read()
            .should_dedup_text_attribute()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyLanguageAssetSchema {
    #[getter]
    pub fn get_text_attribute_name(&self) -> String {
        self.inner
            .0
            .read()
            .get_text_attribute_name()
            .as_str()
            .into()
    }
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.inner.0.read().should_dedup_text_attribute()
    }
}
