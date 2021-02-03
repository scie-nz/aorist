#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::schema::DataSchema;
use crate::storage_setup::StorageSetup;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable, PythonObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct StaticDataTable {
    pub name: String,
    #[constrainable]
    setup: StorageSetup,
    #[constrainable]
    pub schema: DataSchema,
}
