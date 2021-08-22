#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::long_tabular_schema::*;
use crate::schema::tabular_schema::*;
use crate::template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum TextCorpusSource {
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    LongTabularSchema(AoristRef<LongTabularSchema>),
}

impl TextCorpusSource {
    pub fn should_dedup_text_attribute(&self, attr: &String) -> bool {
        match self {
            TextCorpusSource::TabularSchema(_) => false,
            TextCorpusSource::LongTabularSchema(x) => {
                x.0.read().unwrap().should_dedup_text_attribute(attr)
            }
        }
    }
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            TextCorpusSource::TabularSchema(x) => Ok(x
                .0
                .read()
                .unwrap()
                .get_datum_template()
                .0
                .read()
                .unwrap()
                .get_name()),
            TextCorpusSource::LongTabularSchema(x) => Ok(x
                .0
                .read()
                .unwrap()
                .get_datum_template()
                .0
                .read()
                .unwrap()
                .get_name()),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            TextCorpusSource::TabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            TextCorpusSource::LongTabularSchema(x) => x.0.read().unwrap().get_attribute_names(),
        }
    }
}

#[aorist]
pub struct TextCorpusSchema {
    text_attribute_name: String,
    source: AoristRef<TextCorpusSource>,
}

impl TextCorpusSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        self.source.0.read().unwrap().get_datum_template_name()
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        self.source.0.read().unwrap().get_attribute_names()
    }
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.source
            .0
            .read()
            .unwrap()
            .should_dedup_text_attribute(&self.text_attribute_name)
    }
    pub fn get_text_attribute_name(&self) -> String {
        self.text_attribute_name.clone()
    }
}
#[cfg(feature = "python")]
#[pymethods]
impl PyTextCorpusSchema {
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.inner.0.read().unwrap().should_dedup_text_attribute()
    }
}
