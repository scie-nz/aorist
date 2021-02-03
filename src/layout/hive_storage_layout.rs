#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct StaticHiveTableLayout {}

#[pymethods]
impl StaticHiveTableLayout {
    #[new]
    fn new() -> Self {
        Self {
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}

#[aorist_concept]
pub struct DailyGranularity {}
#[pymethods]
impl DailyGranularity {
    #[new]
    fn new() -> Self {
        Self {
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type")]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(DailyGranularity),
}

#[aorist_concept]
pub struct DynamicHiveTableLayout {
    #[constrainable]
    granularity: Granularity,
}
#[pymethods]
impl DynamicHiveTableLayout {
    #[new]
    fn new(granularity: Granularity) -> Self {
        Self {
            granularity,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type")]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
    DynamicHiveTableLayout(DynamicHiveTableLayout),
}
