#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::storage::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct RemoteImportStorageSetup {
    #[constrainable]
    remote: Storage,
    #[constrainable]
    local: Vec<Storage>,
    pub tmp_dir: String,
}
