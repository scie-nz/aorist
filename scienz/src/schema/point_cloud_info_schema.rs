use crate::asset::*;
use crate::attributes::*;
use crate::schema::derived_asset_schema::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::{attribute, derived_schema, AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

derived_schema! {
    name: PointCloudInfoSchema,
    source: PointCloudAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      json: FreeText("JSON for pdal info", false)
}
