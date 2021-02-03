#![allow(non_snake_case)]

use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::storage::hive_table_storage::HiveTableStorage;
use crate::storage::remote_website_storage::RemoteStorage;
use aorist_concept::{aorist_concept2, Constrainable};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum Storage {
    #[constrainable]
    RemoteStorage(RemoteStorage),
    #[constrainable]
    HiveTableStorage(HiveTableStorage),
}
