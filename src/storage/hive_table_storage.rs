#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::encoding::Encoding;
use crate::layout::HiveStorageLayout;
use crate::location::HiveLocation;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct HiveTableStorage {
    #[constrainable]
    location: HiveLocation,
    #[constrainable]
    layout: HiveStorageLayout,
    #[constrainable]
    pub encoding: Encoding,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
#[pymethods]
impl HiveTableStorage {
    #[new]
    fn new(location: HiveLocation, layout: HiveStorageLayout, encoding: Encoding) -> Self {
        Self {
            location,
            layout,
            encoding,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
