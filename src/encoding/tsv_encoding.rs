#![allow(non_snake_case)]

use crate::compression::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::header::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
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
