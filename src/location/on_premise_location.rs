use crate::concept::{AoristConcept, Concept};
use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use crate::location::sqlite_location::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum OnPremiseLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
    #[constrainable]
    SQLiteLocation(SQLiteLocation),
}
