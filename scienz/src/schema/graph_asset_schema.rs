use crate::attributes::*;
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::schema::edgelist_filter_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{schema_enum, AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

schema_enum! {
    name: GraphAssetSchema
    variants:
    - EdgelistFilterSchema
}
