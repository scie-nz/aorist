#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::encoding::Encoding;
use crate::layout::HiveStorageLayout;
use crate::location::HiveLocation;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct HiveTableStorage {
    #[constrainable]
    location: HiveLocation,
    #[constrainable]
    layout: HiveStorageLayout,
    #[constrainable]
    pub encoding: Encoding,
}
