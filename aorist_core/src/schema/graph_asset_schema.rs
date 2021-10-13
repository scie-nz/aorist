use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::edgelist_filter_schema::*;
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::schema_enum;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
#[cfg(feature = "python")]
use pyo3::prelude::*;

schema_enum! {
    name: GraphAssetSchema
    variants:
    - EdgelistFilterSchema
}
