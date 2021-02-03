#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::header::FileHeader;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use aorist_concept::{aorist_concept, Constrainable};

#[aorist_concept]
pub struct CSVEncoding {
    #[constrainable]
    compression: DataCompression,
    #[constrainable]
    header: FileHeader,
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
