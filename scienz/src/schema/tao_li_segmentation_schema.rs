use crate::asset::*;
use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::FloatValue;
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
    name: TAOLiSegmentationSchema,
    sources:
      - point_cloud: PointCloudAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false)
    fields:
      dt1: FloatValue,
      dt2: FloatValue,
      r: FloatValue,
      z_u: FloatValue,
      hmin: FloatValue,
      speed_up: FloatValue
}
