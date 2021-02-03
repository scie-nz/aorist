#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::header::FileHeader;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct TSVEncoding {
    #[constrainable]
    compression: DataCompression,
    #[constrainable]
    header: FileHeader,
}
#[pymethods]
impl TSVEncoding {
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
