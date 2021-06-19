pub mod alluxio;
pub mod aws;
pub mod gcp;
pub mod gitea;
pub mod minio;
pub mod postgres;
pub mod presto;
pub mod ranger;

use crate::constraint::Constraint;
use crate::{AoristConcept, ConceptEnum};
pub use alluxio::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
pub use aws::*;
use derivative::Derivative;
pub use gcp::*;
pub use gitea::*;
pub use minio::*;
use paste::paste;
pub use postgres::*;
pub use presto::*;
use pyo3::prelude::*;
pub use ranger::*;
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
    #[constrainable]
    #[py_default = "None"]
    pub gcp: Option<GCPConfig>,
    #[constrainable]
    #[py_default = "None"]
    pub aws: Option<AWSConfig>,
}
