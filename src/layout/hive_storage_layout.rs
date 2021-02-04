#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct StaticHiveTableLayout {}

#[aorist_concept2]
pub struct DailyGranularity {}

#[aorist_concept2]
pub enum Granularity {
    #[constrainable]
    DailyGranularity(DailyGranularity),
}

#[aorist_concept2]
pub struct DynamicHiveTableLayout {
    #[constrainable]
    granularity: Granularity,
}

#[aorist_concept2]
pub enum HiveStorageLayout {
    StaticHiveTableLayout(StaticHiveTableLayout),
    DynamicHiveTableLayout(DynamicHiveTableLayout),
}
