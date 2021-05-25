use crate::concept::{AoristConcept, AoristConceptChildren, WrappedConcept, ConceptEnum, Concept};
use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use crate::location::s3_location::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
    #[constrainable]
    S3Location(S3Location),
}
