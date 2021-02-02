#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::header::FileHeader;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct CSVEncoding {
    #[constrainable]
    compression: DataCompression,
    #[constrainable]
    header: FileHeader,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
#[pymethods]
impl CSVEncoding {
    #[new]
    fn new(compression: DataCompression, header: FileHeader) -> Self {
        Self {
            compression,
            header,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
