#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct MinioConfig {
    pub server: String,
    pub port: usize,
    pub bucket: String,
    pub access_key: String,
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
    #[getter]
    fn server(&self) -> String {
        self.server.clone()
    }
    #[getter]
    fn port(&self) -> usize {
        self.port
    }
    #[getter]
    fn bucket(&self) -> String {
        self.bucket.clone()
    }
    #[getter]
    fn access_key(&self) -> String {
        self.access_key.clone()
    }
    #[getter]
    fn secret_key(&self) -> String {
        self.secret_key.clone()
    }
}
