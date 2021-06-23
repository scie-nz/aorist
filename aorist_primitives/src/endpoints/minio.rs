#[cfg(feature = "python")]
use serde::{Serialize, Deserialize};
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct MinioConfig {
    pub server: String,
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}
#[cfg_attr(feature = "python", pymethods)]
impl MinioConfig {
    #[new]
    fn new(
        server: String,
        port: usize,
        bucket: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        MinioConfig { server, port, bucket, access_key, secret_key }
    }
}
