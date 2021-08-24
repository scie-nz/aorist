#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::text_corpus_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct FasttextEmbeddingSchema {
    pub dim: usize,
    #[constrainable]
    pub source_schema: AoristRef<TextCorpusSchema>,
    pub datum_template: AoristRef<DatumTemplate>,
}
impl FasttextEmbeddingSchema {
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        self.source_schema.clone()
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        self.datum_template
            .0
            .read()
            .unwrap()
            .get_attributes()
            .iter()
            .map(|x| x.get_name())
            .collect()
    }
}
impl FasttextEmbeddingSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}
