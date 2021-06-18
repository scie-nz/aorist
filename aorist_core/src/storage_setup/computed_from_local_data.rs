#![allow(non_snake_case)]
use crate::storage::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[aorist]
pub struct ComputedFromLocalData {
    pub source_asset_names: BTreeMap<String, String>,
    #[constrainable]
    pub target: Storage,
    pub tmp_dir: String,
}
