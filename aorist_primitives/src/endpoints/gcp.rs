use crate::concept::{AOption, AString};
use abi_stable::std_types::ROption;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct GCPConfig {
    pub use_default_credentials: bool,
    pub service_account_file: AOption<AString>,
    pub project_name: AString,
    pub data_location: AString,
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
            service_account_file: AOption(match service_account_file {
                None => ROption::RNone,
                Some(x) => ROption::RSome(x.as_str().into()),
            }),
            project_name: project_name.as_str().into(),
            data_location: data_location.as_str().into(),
        }
    }
    #[getter]
    fn project_name(&self) -> String {
        self.project_name.to_string()
    }
    #[getter]
    fn data_location(&self) -> String {
        self.data_location.to_string()
    }
    #[getter]
    fn service_account_file(&self) -> Option<String> {
        match self.service_account_file.0 {
            ROption::RSome(ref x) => Some(x.to_string()),
            ROption::RNone => None,
        }
    }
    #[getter]
    fn use_default_credentials(&self) -> bool {
        self.use_default_credentials
    }
}
