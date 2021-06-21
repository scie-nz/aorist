use crate::encoding::*;
use crate::storage::bigquery_storage::*;
use crate::storage::git_storage::*;
use crate::storage::hive_table_storage::*;
use crate::storage::local_file_storage::*;
use crate::storage::postgres_storage::*;
use crate::storage::remote_storage::*;
use crate::storage::sqlite_storage::*;
use crate::concept::{AoristRef, WrappedConcept};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
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
}

impl Storage {
    pub fn get_encoding(&self) -> Option<AoristRef<Encoding>> {
        match &self {
            Self::RemoteStorage(x) => Some(x.0.read().unwrap().encoding.clone()),
            Self::HiveTableStorage(x) => Some(x.0.read().unwrap().encoding.clone()),
            Self::LocalFileStorage(x) => Some(x.0.read().unwrap().encoding.clone()),
            Self::GitStorage(x) => Some(x.0.read().unwrap().encoding.clone()),
            Self::SQLiteStorage(_) => None,
            Self::PostgresStorage(_) => None,
            Self::BigQueryStorage(_) => None,
        }
    }
}
