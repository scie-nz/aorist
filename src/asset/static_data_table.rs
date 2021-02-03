#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct StaticDataTable {
    pub name: String,
    #[constrainable]
    setup: StorageSetup,
    #[constrainable]
    pub schema: DataSchema,
}
#[pymethods]
impl StaticDataTable {
    #[new]
    fn new(name: String, setup: StorageSetup, schema: DataSchema) -> Self {
        Self {
            name,
            setup,
            schema,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
