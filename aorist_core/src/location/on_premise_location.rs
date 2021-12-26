use crate::concept::{AoristRef, WrappedConcept};
use crate::location::alluxio_location::*;
use crate::location::local_file_system_location::*;
use crate::location::minio_location::*;
use crate::location::postgres_location::*;
use crate::location::sqlite_location::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec, AoristConceptBase, AoristConcept, ConceptEnum};
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
