use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
}
