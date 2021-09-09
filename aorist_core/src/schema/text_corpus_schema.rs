use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::data_schema::DataSchema;
use crate::schema::derived_asset_schema::DerivedAssetSchema;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{attribute, derived_schema};
use aorist_attributes::*;
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;

derived_schema! { 
    name: TextCorpusSchema,
    source: StaticDataTable,
    attributes:
      document_id: StringIdentifier("document id", false),
      document: FreeText("document text", false)
    fields:
      text_attribute_name: String
}

impl TextCorpusSchema {
    pub fn should_dedup_text_attribute(&self) -> bool {
        match &*self.get_source().0.read().unwrap().get_schema().0.read().unwrap() {
            DataSchema::TabularSchema(_) => false,
            DataSchema::LongTabularSchema(x) => {
                x.0.read().unwrap().should_dedup_text_attribute(&self.text_attribute_name)
            }
            _ => panic!("DataSchema must be either TabularSchema or LongTabularSchema"),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTextCorpusSchema {
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.inner.0.read().unwrap().should_dedup_text_attribute()
    }
}
