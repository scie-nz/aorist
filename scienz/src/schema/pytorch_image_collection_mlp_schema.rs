use crate::asset::*;
use crate::attributes::*;
use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
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
    name: PyTorchImageCollectionMLPSchema,
    sources:
      - features: ImageCorpus,
      - labels: ImageCorpus,
    attributes:
      path: KeyStringIdentifier("File Path", false)
    fields:
      model: AString,
      optimizer: AString,
      train_epochs: usize
}
