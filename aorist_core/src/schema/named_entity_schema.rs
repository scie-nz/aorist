#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::text_corpus_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum NamedEntitySchema {
    #[constrainable]
    SpacyNamedEntitySchema(AoristRef<SpacyNamedEntitySchema>),
}

impl NamedEntitySchema {
    pub fn get_source_schema(&self) -> AoristRef<TextCorpusSchema> {
        match self {
            NamedEntitySchema::SpacyNamedEntitySchema(x) => {
                x.0.read().unwrap().source_schema.clone()
            }
        }
    }
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        match self {
            NamedEntitySchema::SpacyNamedEntitySchema(x) => {
                x.0.read().unwrap().get_datum_template()
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            NamedEntitySchema::SpacyNamedEntitySchema(x) => {
                x.0.read().unwrap().get_attribute_names()
            }
        }
    }
}

#[aorist]
pub struct SpacyNamedEntitySchema {
    pub spacy_model_name: String,
    #[constrainable]
    pub source_schema: AoristRef<TextCorpusSchema>,
    pub datum_template: AoristRef<DatumTemplate>,
}
impl SpacyNamedEntitySchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
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
#[cfg(feature = "python")]
#[pymethods]
impl PySpacyNamedEntitySchema {
    #[getter]
    pub fn datum_template(&self) -> PyDatumTemplate {
        PyDatumTemplate {
            inner: self.inner.0.read().unwrap().get_datum_template().clone(),
        }
    }
}
