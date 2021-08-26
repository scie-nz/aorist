use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::asset::*;
use crate::schema::derived_asset_schema::DerivedAssetSchema;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use crate::attributes::*;
use aorist_primitives::*;
use aorist_attributes::*;
#[cfg(feature = "python")]
use pyo3::prelude::*;

derived_schema! { 
    name: PointCloudBoundarySchema,
    source: StaticDataTable,
    attributes:
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
