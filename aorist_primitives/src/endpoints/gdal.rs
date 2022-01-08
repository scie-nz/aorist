#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::{AString};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct GDALConfig {
    pub gdal_path: AString,
}
#[cfg(feature = "python")]
#[pymethods]
impl GDALConfig {
    #[new]
    fn new(gdal_path: String) -> Self {
        GDALConfig { gdal_path: gdal_path.as_str().into() }
    }
    #[getter]
    pub fn gdal_path(&self) -> String {
        self.gdal_path.to_string()
    }
}
