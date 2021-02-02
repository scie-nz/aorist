#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::schema::tabular_schema::TabularSchema;
use aorist_concept::Constrainable;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum DataSchema {
    #[constrainable]
    TabularSchema(TabularSchema),
}
