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
pub struct FasttextEmbeddingSchema {
    pub dim: usize,
    #[constrainable]
    pub source_schema: AoristRef<TextCorpusSchema>,
}
impl FasttextEmbeddingSchema { 
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        self.source_schema.clone()
    }
}
