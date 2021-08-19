#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct GCPConfig {
    pub use_default_credentials: bool,
    pub service_account_file: Option<String>,
    pub project_name: String,
    pub data_location: String,
}
#[cfg(feature = "python")]
#[pymethods]
impl GCPConfig {
    #[new]
    fn new(
        use_default_credentials: bool,
        service_account_file: Option<String>,
        project_name: String,
        data_location: String,
    ) -> Self {
        GCPConfig {
            use_default_credentials,
            service_account_file,
            project_name,
            data_location,
        }
    }
    #[getter]
    fn project_name(&self) -> String {
        self.project_name.clone()
    }
    #[getter]
    fn data_location(&self) -> String {
        self.data_location.clone()
    }
    #[getter]
    fn service_account_file(&self) -> Option<String> {
        self.service_account_file.clone()
    }
    #[getter]
    fn use_default_credentials(&self) -> bool {
        self.use_default_credentials
    }
}
