#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::tabular_schema::*;
use crate::schema::long_tabular_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum TextCorpusSchema {
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    LongTabularSchema(AoristRef<LongTabularSchema>),
}

impl TextCorpusSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            TextCorpusSchema::TabularSchema(x) => Ok(x.0.read().unwrap().datumTemplateName.clone()),
            TextCorpusSchema::LongTabularSchema(x) => Ok(x.0.read().unwrap().datumTemplateName.clone()),
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            TextCorpusSchema::TabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            TextCorpusSchema::LongTabularSchema(x) => x.0.read().unwrap().get_attribute_names(),
        }
    }
}
