use crate::concept::{AoristRef, WrappedConcept};
use crate::location::alluxio_location::*;
use crate::location::minio_location::*;
use crate::location::s3_location::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
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
