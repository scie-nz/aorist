use crate::asset::*;
use crate::attributes::*;
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
    name: FLANNKNNMatchSchema,
    sources:
      - query_descriptors: ImageCorpusKeyPoints,
      - train_descriptors: ImageCorpusKeyPoints,
    attributes:
      path: KeyStringIdentifier("File Path", false),
      matches: FreeText("JSON of matches", false)
    fields:
      kd_tree_index_trees: usize,
      search_checks: usize,
      search_eps: FloatValue,
      sorted: bool,
      explore_all_trees: bool
}
