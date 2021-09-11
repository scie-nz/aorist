#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct PDALConfig {
    pub pdal_binary: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl PDALConfig {
    #[new]
    fn new(
        pdal_binary: String,
    ) -> Self {
        PDALConfig {
            pdal_binary,
        }
    }
    #[getter]
    pub fn pdal_binary(&self) -> String {
        self.pdal_binary.clone()
    }
}
