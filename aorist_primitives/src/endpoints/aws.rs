#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct AWSConfig {
    pub access_key_id: Option<String>,
    pub access_key_secret: Option<String>,
    pub access_key_env_var: Option<String>,
    pub access_key_secret_env_var: Option<String>,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
#[cfg(feature = "python")]
#[pymethods]
impl AWSConfig {
    #[new]
    fn new(
        access_key_id: Option<String>,
        access_key_secret: Option<String>,
        access_key_env_var: Option<String>,
        access_key_secret_env_var: Option<String>,
        region: Option<String>,
        project_name: Option<String>,
    ) -> Self {
        assert!(
            (access_key_id.is_some() && access_key_secret.is_some()) ||
            (access_key_env_var.is_some() && access_key_secret_env_var.is_some())
        );
        AWSConfig {
            access_key_id,
            access_key_secret,
            access_key_env_var,
            access_key_secret_env_var,
            region,
            project_name,
        }
    }
}
