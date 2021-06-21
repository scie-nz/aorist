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

use crate::concept::{AoristRef, WrappedConcept};
use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct EndpointConfig {
    #[constrainable]
    pub presto: Option<AoristRef<PrestoConfig>>,
    #[constrainable]
    pub alluxio: Option<AoristRef<AlluxioConfig>>,
    #[constrainable]
    pub ranger: Option<AoristRef<RangerConfig>>,
    #[constrainable]
    pub gitea: Option<AoristRef<GiteaConfig>>,
    #[constrainable]
    pub minio: Option<AoristRef<MinioConfig>>,
    #[constrainable]
    pub postgres: Option<AoristRef<PostgresConfig>>,
    #[constrainable]
    pub gcp: Option<AoristRef<GCPConfig>>,
    #[constrainable]
    pub aws: Option<AoristRef<AWSConfig>>,
}
