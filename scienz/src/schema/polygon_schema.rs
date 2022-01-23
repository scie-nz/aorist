use crate::attributes::*;
use crate::template::*;
use abi_stable::std_types::ROption;
use aorist_attributes::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_primitives::{attribute, primary_schema};
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use derivative::Derivative;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

primary_schema! {
    name: PolygonSchema,
    attributes:
      id: KeyStringIdentifier("Polygon Identifier", false),
      name: FreeText("Polygon name", true),
      wkt: WKTString("WKT string", false)
}
