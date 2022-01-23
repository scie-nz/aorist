use crate::asset::*;
use crate::attributes::*;
use crate::schema::data_schema::DataSchema;
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_primitives::{attribute, derived_schema};
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

derived_schema! {
    name: TextCorpusSchema,
    sources: StaticDataTable,
    attributes:
      document_id: StringIdentifier("document id", false),
      document: FreeText("document text", false)
    fields:
      text_attribute_name: AString
}

impl TextCorpusSchema {
    pub fn should_dedup_text_attribute(&self) -> bool {
        for source in &*self.get_sources() {
            let dedup = match &*source.get_schema().0.read() {
                DataSchema::TabularSchema(_) => false,
                DataSchema::LongTabularSchema(x) => {
                    x.0.read()
                        .should_dedup_text_attribute(&self.text_attribute_name)
                }
                _ => panic!("DataSchema must be either TabularSchema or LongTabularSchema"),
            };
            if dedup {
                return true;
            }
        }
        false
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTextCorpusSchema {
    pub fn should_dedup_text_attribute(&self) -> bool {
        self.inner.0.read().should_dedup_text_attribute()
    }
}
