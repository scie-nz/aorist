use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::encoding::*;
use crate::storage::bigquery_storage::*;
use crate::storage::git_storage::*;
use crate::storage::hive_table_storage::*;
use crate::storage::local_file_storage::*;
use crate::storage::postgres_storage::*;
use crate::storage::remote_storage::*;
use crate::storage::sqlite_storage::*;

#[aorist]
pub enum Storage {
    #[constrainable]
    RemoteStorage(RemoteStorage),
    #[constrainable]
    GitStorage(GitStorage),
    #[constrainable]
    HiveTableStorage(HiveTableStorage),
    #[constrainable]
    LocalFileStorage(LocalFileStorage),
    #[constrainable]
    SQLiteStorage(SQLiteStorage),
    #[constrainable]
    PostgresStorage(PostgresStorage),
    #[constrainable]
    BigQueryStorage(BigQueryStorage),
}

impl Storage {
    pub fn get_encoding(&self) -> Option<Encoding> {
        match &self {
            Self::RemoteStorage(x) => Some(x.encoding.clone()),
            Self::HiveTableStorage(x) => Some(x.encoding.clone()),
            Self::LocalFileStorage(x) => Some(x.encoding.clone()),
            Self::GitStorage(x) => Some(x.encoding.clone()),
            Self::SQLiteStorage(_) => None,
            Self::PostgresStorage(_) => None,
            Self::BigQueryStorage(_) => None,
        }
    }
}
