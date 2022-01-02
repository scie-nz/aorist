use crate::concept::WrappedConcept;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AoristRef;
use aorist_primitives::{
    AString, AVec, AWSConfig, AlluxioConfig, AoristConcept, AoristConceptBase, ConceptEnum,
    DaskConfig, GCPConfig, GDALConfig, GiteaConfig, LINZAPIConfig, MinioConfig, PDALConfig,
    PostgresConfig, PrestoConfig, RangerConfig, TPrestoEndpoints,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct EndpointConfig {
    pub presto: AOption<PrestoConfig>,
    pub alluxio: AOption<AlluxioConfig>,
    pub ranger: AOption<RangerConfig>,
    pub gitea: AOption<GiteaConfig>,
    pub minio: AOption<MinioConfig>,
    pub postgres: AOption<PostgresConfig>,
    pub gcp: AOption<GCPConfig>,
    pub aws: AOption<AWSConfig>,
    pub pdal: AOption<PDALConfig>,
    pub linz: AOption<LINZAPIConfig>,
    pub dask: AOption<DaskConfig>,
    pub gdal: AOption<GDALConfig>,
}

impl TPrestoEndpoints for EndpointConfig {
    fn presto_config(&self) -> PrestoConfig {
        self.presto.as_ref().unwrap().clone()
    }
}
