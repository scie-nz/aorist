use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct LongTabularSchema {
    pub datum_template: AoristRef<DatumTemplate>,
    pub key_attributes: AVec<AString>,
    pub value_attributes: AVec<AString>,
}
impl LongTabularSchema {
    pub fn get_datum_template(&self) -> AoristRef<DatumTemplate> {
        self.datum_template.clone()
    }
}

impl LongTabularSchema {
    pub fn get_attribute_names(&self) -> AVec<AString> {
        self.key_attributes
            .clone()
            .into_iter()
            .chain(self.value_attributes.clone().into_iter())
            .collect()
    }
    pub fn should_dedup_text_attribute(&self, attr: &AString) -> bool {
        for attribute in self.key_attributes.iter() {
            if attr == attribute {
                return true;
            }
        }
        false
    }
}
