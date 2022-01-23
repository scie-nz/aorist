use crate::asset::*;
use crate::attributes::*;
use crate::schema::derived_asset_schema::*;
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
    name: HomographyFromKNNMatchSchema,
    source: ImageCorporaKNNMatch,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      src_points: FreeText("JSON of source points", false),
      dst_points: FreeText("JSON of destination points", false),
      matrix: FreeText("JSON of Homography Matrix (H)", false)
    fields:
      knn_filter_threshold: FloatValue,
      method: AString,
      ransac_reproj_threshold: AOption<FloatValue>,
      max_iters: usize,
      confidence: FloatValue
}
