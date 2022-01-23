use aorist_util::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct MinioConfig {
    pub server: AString,
    pub port: usize,
    pub bucket: AString,
    pub access_key: AString,
    pub secret_key: AString,
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
            server: server.as_str().into(),
            port,
            bucket: bucket.as_str().into(),
            access_key: access_key.as_str().into(),
            secret_key: secret_key.as_str().into(),
        }
    }
    #[getter]
    fn server(&self) -> String {
        self.server.to_string()
    }
    #[getter]
    fn port(&self) -> usize {
        self.port
    }
    #[getter]
    fn bucket(&self) -> String {
        self.bucket.to_string()
    }
    #[getter]
    fn access_key(&self) -> String {
        self.access_key.to_string()
    }
    #[getter]
    fn secret_key(&self) -> String {
        self.secret_key.to_string()
    }
}
