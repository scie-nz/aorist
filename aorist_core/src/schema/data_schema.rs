#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use crate::schema::tabular_schema::*;
use crate::schema::time_ordered_tabular_schema::*;
use crate::schema::undefined_tabular_schema::*;
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum DataSchema {
    #[constrainable]
    TabularSchema(AoristRef<TabularSchema>),
    #[constrainable]
    TimeOrderedTabularSchema(AoristRef<TimeOrderedTabularSchema>),
    #[constrainable]
    UndefinedTabularSchema(AoristRef<UndefinedTabularSchema>),
}

impl DataSchema {
    pub fn get_datum_template_name(&self) -> Result<String, String> {
        match self {
            DataSchema::TabularSchema(x) => Ok(x.0.read().unwrap().datumTemplateName.clone()),
            DataSchema::TimeOrderedTabularSchema(x) => Ok(x.0.read().unwrap().datumTemplateName.clone()),
            DataSchema::UndefinedTabularSchema(_) => {
                Err("UndefinedTabularSchema has no datum template.".to_string())
            }
        }
    }
    pub fn get_attribute_names(&self) -> Vec<String> {
        match self {
            DataSchema::TabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::TimeOrderedTabularSchema(x) => x.0.read().unwrap().attributes.clone(),
            DataSchema::UndefinedTabularSchema(_) => vec![],
        }
    }
}
