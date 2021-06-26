#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
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
        GCPConfig { use_default_credentials, service_account_file, project_name, data_location}  
    }
}
