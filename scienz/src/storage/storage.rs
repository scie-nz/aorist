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
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::AoristRef;
use aorist_util::{AString, AVec};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
    pub fn get_encoding(&self) -> AOption<AoristRef<Encoding>> {
        match &self {
            Self::RemoteStorage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::HiveTableStorage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::LocalFileStorage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::GitStorage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::InlineBlobStorage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::S3Storage(x) => AOption(ROption::RSome(x.0.read().encoding.clone())),
            Self::SQLiteStorage(_) => AOption(ROption::RNone),
            Self::PostgresStorage(_) => AOption(ROption::RNone),
            Self::BigQueryStorage(_) => AOption(ROption::RNone),
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyStorage {
    #[getter]
    pub fn encoding(&self) -> Option<PyEncoding> {
        match self.inner.0.read().get_encoding() {
            AOption(ROption::RSome(x)) => Some(PyEncoding { inner: x }),
            AOption(ROption::RNone) => None,
        }
    }
}
