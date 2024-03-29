use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{
    AWSConfig, AlluxioConfig, AoristConceptBase, ConceptEnum, DaskConfig, GCPConfig,
    GDALConfig, GiteaConfig, LINZAPIConfig, MinioConfig, PDALConfig, PostgresConfig, PrestoConfig,
    RangerConfig, TPrestoEndpoints,
};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec, AoristRef};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
