#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::*;
use crate::encoding::*;
use crate::layout::*;
use crate::location::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct HiveTableStorage {
    #[constrainable]
    location: HiveLocation,
    #[constrainable]
    layout: HiveStorageLayout,
    #[constrainable]
    pub encoding: Encoding,
}
