#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct DaskConfig {
    pub server: String,
    pub port: usize,
}
#[cfg(feature = "python")]
#[pymethods]
impl DaskConfig {
    #[new]
    fn new(server: String, port: usize) -> Self {
        DaskConfig { server, port }
    }
    #[getter]
    fn server(&self) -> String {
        self.server.clone()
    }
    #[getter]
    fn port(&self) -> usize {
        self.port
    }
}
