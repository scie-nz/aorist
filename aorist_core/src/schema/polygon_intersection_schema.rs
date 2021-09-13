use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::derived_asset_schema::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::{attribute, derived_schema};
use aorist_attributes::*;
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;

derived_schema! { 
    name: PolygonIntersectionSchema,
    sources: GeospatialAsset,
    attributes:
      polygon_ids: JSON("list with polygon identifiers", false),
      overlap: JSON("list with overlap between i-th and j-th polygon", false)
}
