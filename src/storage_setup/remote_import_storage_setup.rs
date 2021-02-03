#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::storage::Storage;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct RemoteImportStorageSetup {
    #[constrainable]
    remote: Storage,
    #[constrainable]
    local: Vec<Storage>,
    pub tmp_dir: String,
}

#[pymethods]
impl RemoteImportStorageSetup {
    #[new]
    fn new(remote: Storage, local: Vec<Storage>, tmp_dir: String) -> Self {
        Self {
            remote,
            local,
            tmp_dir,
            uuid: None,
            tag: None,
            constraints: Vec::new(),
        }
    }
}
