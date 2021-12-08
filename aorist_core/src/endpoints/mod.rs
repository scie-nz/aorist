use crate::concept::{AoristRef, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{
    AString, AVec, AWSConfig, AlluxioConfig, AoristConcept, ConceptEnum, DaskConfig, GCPConfig,
    GDALConfig, GiteaConfig, LINZAPIConfig, MinioConfig, PDALConfig, PostgresConfig, PrestoConfig,
    RangerConfig, TPrestoEndpoints,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
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
    pub dask: Option<DaskConfig>,
    pub gdal: Option<GDALConfig>,
}

impl TPrestoEndpoints for EndpointConfig {
    fn presto_config(&self) -> PrestoConfig {
        self.presto.as_ref().unwrap().clone()
    }
}
