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
    name: PerspectiveTransformedGriddedPolygonCollectionSchema, 
    sources: 
      - polygon_to_grid: PolygonIntersectionAsset,
      - perspective_transform: GridPerspectiveTransformAsset, 
    attributes:
      polygon_id: KeyStringIdentifier("Polygon Identifier", false),
      grid_cell_id: KeyStringIdentifier("PolygonIdentifier", false),
      name: FreeText("Polygon name", true),
      original_wkt: WKTString("WKT string", false),
      wkt: WKTString("Transformed WKT string", false)
}
