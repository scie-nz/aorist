use crate::asset::*;
use crate::attributes::*;
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: PolygonIntersectionSchema,
    sources: GeospatialAsset,
    attributes:
      polygon_ids: JSON("list with polygon identifiers", false),
      overlap: JSON("list with overlap between i-th and j-th polygon", false)
}
