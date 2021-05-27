use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::location::alluxio_location::*;
use crate::location::local_file_system_location::*;
use crate::location::minio_location::*;
use crate::location::postgres_location::*;
use crate::location::sqlite_location::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum OnPremiseLocation {
    #[constrainable]
    AlluxioLocation(AlluxioLocation),
    #[constrainable]
    LocalFileSystemLocation(LocalFileSystemLocation),
    #[constrainable]
    MinioLocation(MinioLocation),
    #[constrainable]
    SQLiteLocation(SQLiteLocation),
    #[constrainable]
    PostgresLocation(PostgresLocation),
}
