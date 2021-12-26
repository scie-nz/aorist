use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConceptBase, AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::FloatValue;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

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
