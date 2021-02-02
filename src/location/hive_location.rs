use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::location::alluxio_location::AlluxioLocation;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use pyo3::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
}
