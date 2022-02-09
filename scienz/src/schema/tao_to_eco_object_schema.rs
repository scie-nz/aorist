use crate::asset::*;
use crate::attributes::*;
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
    name: TAOToEcoObjectSchema,
    sources:
      - taos: PolygonCollectionAsset,
      - adjacency: SimpleUndirectedGraphAsset,
    attributes:
      path: KeyStringIdentifier("TAO File Path", false),
      tao_id: KeyInt64Identifier("ID of TAO (unique in file)", false),
      eco_object_id: KeyInt64Identifier("ID of TAO (unique in file)", false),
      wkt: WKTString("WKT string of TAO boundary", false),
      metrics: JSON("JSON map of metrics", false)
    fields:
      pruning_threshold: FloatValue,
      max_neck_size: FloatValue,
      minimum_mapping_area: FloatValue
}
