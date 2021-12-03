use crate::asset::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::Encoding;
use crate::schema::*;
use crate::storage::Storage;
use crate::storage_setup::*;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{asset_enum, AoristConcept, ConceptEnum};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

asset_enum! {
    name: GraphAsset
    variants:
    - SimpleUndirectedGraphAsset
}
