use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use crate::location::s3_location::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum HiveLocation {
    #[constrainable]
    AlluxioLocation(AoristRef<AlluxioLocation>),
    #[constrainable]
    MinioLocation(AoristRef<MinioLocation>),
    #[constrainable]
    S3Location(AoristRef<S3Location>),
}
