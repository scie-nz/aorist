use crate::location::alluxio_location::*;
use crate::location::local_file_system_location::*;
use crate::location::minio_location::*;
use crate::location::postgres_location::*;
use crate::location::sqlite_location::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
