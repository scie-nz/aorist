#![allow(non_snake_case)]

use crate::attributes::Attribute;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::template::datum_template::TDatumTemplate;

use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct KeyedStruct {
    pub name: String,
    #[constrainable]
    attributes: Vec<Attribute>,
}

#[pymethods]
impl KeyedStruct {
    #[new]
    fn new(name: String, attributes: Vec<Attribute>) -> Self {
        Self {
            name,
            attributes,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
impl TDatumTemplate for KeyedStruct {
    fn get_attributes(&self) -> Vec<Attribute> {
        self.attributes.clone()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
