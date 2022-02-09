use crate::asset::*;
use crate::attributes::*;
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{attribute, derived_schema};
use aorist_primitives::{AoristConceptBase, ConceptEnum};
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
    name: SIFTAffineImageKeyPointSchema,
    source: ImageCorpus,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      key_points: FreeText("JSON of key points", false)
    fields:
      nfeatures: usize,
      n_octave_layers: usize,
      contrast_threshold: FloatValue,
      edge_threshold: FloatValue,
      sigma: FloatValue,
      min_tilt: i32,
      max_tilt: i32,
      tilt_step: FloatValue,
      rotate_step_base: FloatValue
}
