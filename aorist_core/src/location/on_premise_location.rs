use crate::location::alluxio_location::*;
use crate::location::local_file_system_location::*;
use crate::location::minio_location::*;
use crate::location::postgres_location::*;
use crate::location::sqlite_location::*;
use crate::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum OnPremiseLocation {
    #[constrainable]
    AlluxioLocation(AoristRef<AlluxioLocation>),
    #[constrainable]
    LocalFileSystemLocation(AoristRef<LocalFileSystemLocation>),
    #[constrainable]
    MinioLocation(AoristRef<MinioLocation>),
    #[constrainable]
    SQLiteLocation(AoristRef<SQLiteLocation>),
    #[constrainable]
    PostgresLocation(AoristRef<PostgresLocation>),
}
