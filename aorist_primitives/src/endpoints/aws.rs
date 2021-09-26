#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct AWSConfig {
    pub aws_key_id: Option<String>,
    pub aws_key_secret: Option<String>,
    pub aws_key_env_var: Option<String>,
    pub aws_key_secret_env_var: Option<String>,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
#[cfg(feature = "python")]
#[pymethods]
impl AWSConfig {
    #[new]
    fn new(
        aws_key_id: Option<String>,
        aws_key_secret: Option<String>,
        aws_key_env_var: Option<String>,
        aws_key_secret_env_var: Option<String>,
        region: Option<String>,
        project_name: Option<String>,
    ) -> Self {
        assert!(
            (aws_key_id.is_some() && aws_key_secret.is_some())
                || (aws_key_env_var.is_some() && aws_key_secret_env_var.is_some())
        );
        AWSConfig {
            aws_key_id,
            aws_key_secret,
            aws_key_env_var,
            aws_key_secret_env_var,
            region,
            project_name,
        }
    }
    #[getter]
    pub fn aws_key_id(&self) -> Option<String> {
        self.aws_key_id.clone()
    }
    #[getter]
    pub fn aws_key_secret(&self) -> Option<String> {
        self.aws_key_secret.clone()
    }
    #[getter]
    pub fn aws_key_env_var(&self) -> Option<String> {
        self.aws_key_env_var.clone()
    }
    #[getter]
    pub fn aws_key_secret_env_var(&self) -> Option<String> {
        self.aws_key_secret_env_var.clone()
    }
    #[getter]
    pub fn region(&self) -> Option<String> {
        self.region.clone()
    }
    #[getter]
    pub fn project_name(&self) -> Option<String> {
        self.project_name.clone()
    }
}
