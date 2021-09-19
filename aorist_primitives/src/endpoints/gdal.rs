#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct GDALConfig {
    pub gdal_path: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl GDALConfig {
    #[new]
    fn new(gdal_path: String) -> Self {
        GDALConfig { gdal_path }
    }
    #[getter]
    pub fn gdal_path(&self) -> String {
        self.gdal_path.clone()
    }
}
