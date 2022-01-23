use crate::asset::*;
use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::FloatValue;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_primitives::{attribute, derived_schema};
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

derived_schema! {
    name: TAOMarkerControlledWatershedSegmentationSchema,
    sources:
      - point_cloud: PointCloudAsset,
      - chm: RasterAsset,
      - ttops: PointCloudAsset,
    attributes:
      prefix: KeyStringIdentifier("File Prefix", false)
    fields:
      th_tree: FloatValue
}
