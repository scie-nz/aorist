#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct MinioConfig {
    #[pyo3(get, set)]
    pub server: String,
    #[pyo3(get, set)]
    pub port: usize,
    #[pyo3(get, set)]
    pub bucket: String,
    #[pyo3(get, set)]
    pub access_key: String,
    #[pyo3(get, set)]
    pub secret_key: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl MinioConfig {
    #[new]
    fn new(
        server: String,
        port: usize,
        bucket: String,
        access_key: String,
        secret_key: String,
    ) -> Self {
        MinioConfig {
            server,
            port,
            bucket,
            access_key,
            secret_key,
        }
    }
}
