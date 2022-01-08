use crate::asset::*;
use crate::attributes::*;
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use crate::template::*;
use abi_stable::std_types::ROption;
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
use aorist_primitives::AUuid;

derived_schema! {
    name: LabeledPointCloudSchema,
    sources:
      - point_cloud: PointCloudAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false)
}
