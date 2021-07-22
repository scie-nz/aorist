#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PrestoConfig {
    pub server: String,
    pub http_port: usize,
    pub user: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl PrestoConfig {
    #[new]
    fn new(server: String, http_port: usize, user: String) -> Self {
        PrestoConfig {
            server,
            http_port,
            user,
        }
    }
    #[getter]
    fn user(&self) -> String {
        self.user.clone()
    }
    #[getter]
    fn http_port(&self) -> usize {
        self.http_port
    }
    #[getter]
    fn server(&self) -> String {
        self.server.clone()
    }
}
