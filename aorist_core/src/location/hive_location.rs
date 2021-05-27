use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use crate::location::s3_location::*;

#[aorist]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
    #[constrainable]
    S3Location(S3Location),
}
