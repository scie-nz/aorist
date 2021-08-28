use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{
    AWSConfig, AlluxioConfig, AoristConcept, ConceptEnum, GCPConfig, GiteaConfig, MinioConfig,
    PostgresConfig, PrestoConfig, RangerConfig, TPrestoEndpoints, PDALConfig, LINZAPIConfig,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist(derivative(Hash))]
pub struct EndpointConfig {
    pub presto: Option<PrestoConfig>,
    pub alluxio: Option<AlluxioConfig>,
    pub ranger: Option<RangerConfig>,
    pub gitea: Option<GiteaConfig>,
    pub minio: Option<MinioConfig>,
    pub postgres: Option<PostgresConfig>,
    pub gcp: Option<GCPConfig>,
    pub aws: Option<AWSConfig>,
    pub pdal: Option<PDALConfig>,
    pub linz: Option<LINZAPIConfig>,
}

impl TPrestoEndpoints for EndpointConfig {
    fn presto_config(&self) -> PrestoConfig {
        self.presto.as_ref().unwrap().clone()
    }
}
