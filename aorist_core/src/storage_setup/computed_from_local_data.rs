#![allow(non_snake_case)]
use crate::concept::{AoristRef, WrappedConcept};
use crate::storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ComputedFromLocalData {
    pub source_asset_names: BTreeMap<String, String>,
    #[constrainable]
    pub target: AoristRef<Storage>,
    pub tmp_dir: String,
}
