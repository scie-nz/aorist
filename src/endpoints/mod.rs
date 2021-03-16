pub mod alluxio;
pub mod gitea;
pub mod minio;
pub mod postgres;
pub mod presto;
pub mod ranger;

use crate::concept::Concept;
use crate::constraint::Constraint;
use crate::AoristConcept;
pub use alluxio::{AlluxioConfig, InnerAlluxioConfig};
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
pub use gitea::{GiteaConfig, InnerGiteaConfig};
pub use minio::{InnerMinioConfig, MinioConfig};
use paste::paste;
pub use presto::{InnerPrestoConfig, PrestoConfig};
use pyo3::prelude::*;
pub use ranger::{InnerRangerConfig, RangerConfig};
pub use postgres::{InnerPostgresConfig, PostgresConfig};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept(derivative(Hash))]
pub struct EndpointConfig {
    #[constrainable]
    #[py_default = "None"]
    pub presto: Option<PrestoConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub alluxio: Option<AlluxioConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub ranger: Option<RangerConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub gitea: Option<GiteaConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub minio: Option<MinioConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub postgres: Option<PostgresConfig>,
}
