#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::{AString};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct PrestoConfig {
    pub server: AString,
    pub http_port: usize,
    pub user: AString,
}
#[cfg(feature = "python")]
#[pymethods]
impl PrestoConfig {
    #[new]
    fn new(server: String, http_port: usize, user: String) -> Self {
        PrestoConfig {
            server: server.as_str().into(),
            http_port,
            user: user.as_str().into(),
        }
    }
    #[getter]
    fn user(&self) -> String {
        self.user.to_string()
    }
    #[getter]
    fn http_port(&self) -> usize {
        self.http_port
    }
    #[getter]
    fn server(&self) -> String {
        self.server.to_string()
    }
}
