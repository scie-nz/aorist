use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use crate::schema::derived_asset_schema::*;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

derived_schema! {
    name: HomographyFromKNNMatchSchema,
    source: ImageCorporaKNNMatch,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      src_points: FreeText("JSON of source points", false),
      dst_points: FreeText("JSON of destination points", false),
      matrix: FreeText("JSON of Homography Matrix (H)", false)
    fields:
      knn_filter_threshold: FloatValue,
      method: String,
      ransac_reproj_threshold: Option<FloatValue>,
      max_iters: usize,
      confidence: FloatValue
}
