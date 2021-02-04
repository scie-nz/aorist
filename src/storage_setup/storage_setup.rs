#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::storage_setup::remote_import_storage_setup::*;
use aorist_concept::{aorist_concept, InnerObject, Constrainable};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub enum StorageSetup {
    #[constrainable]
    RemoteImportStorageSetup(RemoteImportStorageSetup),
}
