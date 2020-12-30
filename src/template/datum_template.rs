#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::template::keyed_struct::KeyedStruct;
use crate::template::identifier_tuple::IdentifierTuple;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable)]
#[serde(tag = "type")]
pub enum DatumTemplate {
    KeyedStruct(KeyedStruct),
    IdentifierTuple(IdentifierTuple),
}
impl DatumTemplate {
    pub fn get_name(&self) -> &String {
        match self {
            DatumTemplate::KeyedStruct(x) => x.get_name(),
            DatumTemplate::IdentifierTuple(x) => x.get_name(),
        }
    }
}
