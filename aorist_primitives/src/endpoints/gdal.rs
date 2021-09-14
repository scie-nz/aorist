#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct GDALConfig {
    pub gdal_binary: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl GDALConfig {
    #[new]
    fn new(gdal_binary: String) -> Self {
        GDALConfig { gdal_binary }
    }
    #[getter]
    pub fn gdal_binary(&self) -> String {
        self.gdal_binary.clone()
    }
}
