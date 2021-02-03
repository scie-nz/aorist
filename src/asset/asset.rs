#![allow(non_snake_case)]
use crate::asset::static_data_table::StaticDataTable;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum Asset {
    #[constrainable]
    StaticDataTable(StaticDataTable),
}
