use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::template::*;
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
    name: PointCloudSubsetSchema,
    sources:
      - point_cloud: PointCloudAsset,
      // TODO: this should be more flexible in the future
      - subset: PolygonIntersectionAsset,
    attributes:
      prefix: KeyStringIdentifier("File Prefix", false)
}