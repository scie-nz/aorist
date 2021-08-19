#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::tabular_schema::*;
use crate::schema::long_tabular_schema::*;
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
    pub text_attribute_name: String,
    #[constrainable]
    pub source_schema: AoristRef<TextCorpusSchema>,
}
