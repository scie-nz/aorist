mod alluxio;
mod aws;
mod gcp;
mod gitea;
mod minio;
mod postgres;
mod presto;
mod ranger;

pub use alluxio::*;
pub use aws::*;
pub use gcp::*;
pub use gitea::*;
pub use minio::*;
pub use postgres::*;
pub use presto::*;
pub use ranger::*;

use crate::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct EndpointConfig {
    #[constrainable]
    pub presto: Option<PrestoConfig>,
    #[constrainable]
    pub alluxio: Option<AlluxioConfig>,
    #[constrainable]
    pub ranger: Option<RangerConfig>,
    #[constrainable]
    pub gitea: Option<GiteaConfig>,
    #[constrainable]
    pub minio: Option<MinioConfig>,
    #[constrainable]
    pub postgres: Option<PostgresConfig>,
    #[constrainable]
    pub gcp: Option<GCPConfig>,
    #[constrainable]
    pub aws: Option<AWSConfig>,
}
