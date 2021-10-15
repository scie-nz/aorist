use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::derived_asset_schema::*;
use std::fmt::Debug;
use uuid::Uuid;
use aorist_attributes::*;

derived_schema! { 
    name: AdjacentPolygonsSchema, 
    source: PolygonCollectionAsset,
    attributes:
      id1: KeyInt64Identifier("Polygon 1 Identifier", false),
      id2: KeyInt64Identifier("Polygon 2 Identifier", false)
    fields:
      buffer: Option<FloatValue>
}
