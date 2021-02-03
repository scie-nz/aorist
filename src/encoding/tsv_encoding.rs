#![allow(non_snake_case)]

use crate::compression::DataCompression;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::header::FileHeader;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct TSVEncoding {
    #[constrainable]
    compression: DataCompression,
    #[constrainable]
    header: FileHeader,
}
