#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::storage::Storage;
use aorist_concept::Constrainable;
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[pyclass]
#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct RemoteImportStorageSetup {
    #[constrainable]
    remote: Storage,
    #[constrainable]
    local: Vec<Storage>,
    pub tmp_dir: String,
    uuid: Option<Uuid>,
    tag: Option<String>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
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
