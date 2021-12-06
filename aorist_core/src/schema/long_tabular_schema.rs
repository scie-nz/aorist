use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use aorist_primitives::AString;

#[aorist]
pub struct LongTabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub key_attributes: Vec<AString>,
    pub value_attributes: Vec<AString>,
}
impl LongTabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}

impl LongTabularSchema {
    pub fn get_attribute_names(&self) -> Vec<AString> {
        self.key_attributes
            .clone()
            .into_iter()
            .chain(self.value_attributes.clone().into_iter())
            .collect()
    }
    pub fn should_dedup_text_attribute(&self, attr: &AString) -> bool {
        for attribute in &self.key_attributes {
            if attr == attribute {
                return true;
            }
        }
        false
    }
}
