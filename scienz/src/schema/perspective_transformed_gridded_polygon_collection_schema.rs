use crate::asset::*;
use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
