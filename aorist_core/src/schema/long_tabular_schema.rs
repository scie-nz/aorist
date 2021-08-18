#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct LongTabularSchema {
    pub datumTemplateName: String,
    pub key_attributes: Vec<String>,
    pub value_attributes: Vec<String>,
}

impl LongTabularSchema {
    pub fn get_attribute_names(&self) -> Vec<String> {
        self.key_attributes.clone().into_iter().chain(
            self.value_attributes.clone().into_iter()
        ).collect()
    }
}
