#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::{AString};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct PDALConfig {
    pub pdal_binary: AString,
}
#[cfg(feature = "python")]
#[pymethods]
impl PDALConfig {
    #[new]
    fn new(pdal_binary: String) -> Self {
        PDALConfig { pdal_binary: pdal_binary.as_str().into() }
    }
    #[getter]
    pub fn pdal_binary(&self) -> String {
        self.pdal_binary.to_string()
    }
}
