#![allow(non_snake_case)]
use crate::attributes::*;
use aorist_primitives::{AoristConcept, ConceptEnum};
use crate::concept::{AoristRef, WrappedConcept};
use crate::template::datum_template::TDatumTemplate;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct RowStruct {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<AoristRef<Attribute>>,
}
impl TDatumTemplate for RowStruct {
    fn get_attributes(&self) -> Vec<AoristRef<Attribute>> {
        self.attributes.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
