use crate::attributes::*;
use crate::schema::edgelist_filter_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::{AUuid, AString, AVec};
use aorist_primitives::{schema_enum};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

schema_enum! {
    name: GraphAssetSchema
    variants:
    - EdgelistFilterSchema
}
