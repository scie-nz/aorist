#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::{AString};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct DaskConfig {
    pub server: AString,
    pub port: usize,
}
#[cfg(feature = "python")]
#[pymethods]
impl DaskConfig {
    #[new]
    fn new(server: String, port: usize) -> Self {
        DaskConfig { server: server.as_str().into(), port }
    }
    #[getter]
    fn server(&self) -> String {
        self.server.to_string()
    }
    #[getter]
    fn port(&self) -> usize {
        self.port
    }
}
