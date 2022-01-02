use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum, WrappedConcept};
use crate::schema::derived_asset_schema::*;
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
use uuid::Uuid;

derived_schema! {
    name: NDVISchema,
    source: RasterAsset,
    attributes:
      path: KeyStringIdentifier("File Path", false)
    fields:
      red_band_index: usize,
      nir_band_index: usize
}
