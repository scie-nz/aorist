#![allow(non_snake_case)]
use crate::concept::{AoristConcept, ConceptEnum, WrappedConcept};
use crate::constraint::Constraint;
use crate::storage::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct ComputedFromLocalData {
    pub source_asset_names: BTreeMap<String, String>,
    #[constrainable]
    pub target: Storage,
    pub tmp_dir: String,
}
