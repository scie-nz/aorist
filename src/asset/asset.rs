#![allow(non_snake_case)]
use crate::asset::static_data_table::StaticDataTable;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum Asset {
    #[constrainable]
    StaticDataTable(StaticDataTable),
}
