use crate::concept::{AoristRef, WrappedConcept};
use crate::encoding::*;
use crate::storage::bigquery_storage::*;
use crate::storage::git_storage::*;
use crate::storage::hive_table_storage::*;
use crate::storage::inline_blob_storage::*;
use crate::storage::local_file_storage::*;
use crate::storage::postgres_storage::*;
use crate::storage::remote_storage::*;
use crate::storage::s3_storage::*;
use crate::storage::sqlite_storage::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, ConceptEnum, AString};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub enum Storage {
    #[constrainable]
    RemoteStorage(AoristRef<RemoteStorage>),
    #[constrainable]
    GitStorage(AoristRef<GitStorage>),
    #[constrainable]
    HiveTableStorage(AoristRef<HiveTableStorage>),
    #[constrainable]
    LocalFileStorage(AoristRef<LocalFileStorage>),
    #[constrainable]
    SQLiteStorage(AoristRef<SQLiteStorage>),
    #[constrainable]
    PostgresStorage(AoristRef<PostgresStorage>),
    #[constrainable]
    BigQueryStorage(AoristRef<BigQueryStorage>),
    #[constrainable]
    InlineBlobStorage(AoristRef<InlineBlobStorage>),
    #[constrainable]
    S3Storage(AoristRef<S3Storage>),
}

impl Storage {
    pub fn get_encoding(&self) -> Option<AoristRef<Encoding>> {
        match &self {
            Self::RemoteStorage(x) => Some(x.0.read().encoding.clone()),
            Self::HiveTableStorage(x) => Some(x.0.read().encoding.clone()),
            Self::LocalFileStorage(x) => Some(x.0.read().encoding.clone()),
            Self::GitStorage(x) => Some(x.0.read().encoding.clone()),
            Self::InlineBlobStorage(x) => Some(x.0.read().encoding.clone()),
            Self::S3Storage(x) => Some(x.0.read().encoding.clone()),
            Self::SQLiteStorage(_) => None,
            Self::PostgresStorage(_) => None,
            Self::BigQueryStorage(_) => None,
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStorage {
    #[getter]
    pub fn encoding(&self) -> Option<PyEncoding> {
        self.inner
            .0
            .read()
            .get_encoding()
            .and_then(|x| Some(PyEncoding { inner: x }))
    }
}
