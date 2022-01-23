use crate::asset::*;
use crate::attributes::*;
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_primitives::*;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

derived_schema! {
    name: PointCloudBoundarySchema,
    source: PointCloudInfoAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      area: FloatArea("Area in units", false),
      avg_pt_per_sq_unit: FloatDensity("average points per square unit", false),
      avg_pt_spacing: Float64("TBD", false),
      boundary: WKTString("Boundary as WKT", false),
      boundary_json: JSON("Boyndary as GeoJSON", false),
      density: FloatDensity("density", false),
      edge_length: Float64("TBD", false),
      estimated_edge: Float64("TBD", false),
      hex_offsets: WKTString("TBD", false),
      sample_size: Count("TBD", false),
      threshold: NaturalNumber("TBD", false)
}
