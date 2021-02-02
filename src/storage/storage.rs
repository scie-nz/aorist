#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::storage::hive_table_storage::HiveTableStorage;
use crate::storage::remote_website_storage::RemoteStorage;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type")]
pub enum Storage {
    #[constrainable]
    RemoteStorage(RemoteStorage),
    #[constrainable]
    HiveTableStorage(HiveTableStorage),
}
