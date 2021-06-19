#![allow(non_snake_case)]

use crate::compression::*;
use crate::concept::{AoristConcept, ConceptEnum};
use crate::constraint::*;
use crate::header::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct CSVEncoding {
    #[py_default = "None"]
    #[constrainable]
    pub compression: Option<DataCompression>,
    #[py_default = "None"]
    #[constrainable]
    pub header: Option<FileHeader>,
}
