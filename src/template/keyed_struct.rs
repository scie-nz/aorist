#![allow(non_snake_case)]

use crate::attributes::Attribute;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Serialize, Deserialize, Derivative, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct KeyedStruct {
    pub name: String,
    #[constrainable]
    pub attributes: Vec<Attribute>,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl KeyedStruct {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
