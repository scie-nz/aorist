#[cfg(feature = "python")]
use abi_stable::std_types::ROption;
use aorist_util::{AOption, AString};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct AWSConfig {
    pub aws_key_id: AOption<AString>,
    pub aws_key_secret: AOption<AString>,
    pub aws_key_env_var: AOption<AString>,
    pub aws_key_secret_env_var: AOption<AString>,
    pub region: AString,
    pub project_name: AString,
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
        region: String,
        project_name: String,
    ) -> Self {
        assert!(
            (aws_key_id.is_some() && aws_key_secret.is_some())
                || (aws_key_env_var.is_some() && aws_key_secret_env_var.is_some())
        );
        AWSConfig {
            aws_key_id: AOption(match aws_key_id {
                Some(x) => ROption::RSome(x.as_str().into()),
                None => ROption::RNone,
            }),
            aws_key_secret: AOption(match aws_key_secret {
                Some(x) => ROption::RSome(x.as_str().into()),
                None => ROption::RNone,
            }),
            aws_key_env_var: AOption(match aws_key_env_var {
                Some(x) => ROption::RSome(x.as_str().into()),
                None => ROption::RNone,
            }),
            aws_key_secret_env_var: AOption(match aws_key_secret_env_var {
                Some(x) => ROption::RSome(x.as_str().into()),
                None => ROption::RNone,
            }),
            region: region.as_str().into(),
            project_name: project_name.as_str().into(),
        }
    }
    #[getter]
    fn aws_key_id(&self) -> Option<String> {
        match &self.aws_key_id.0 {
            ROption::RSome(x) => Some(x.to_string()),
            ROption::RNone => None,
        }
    }
    #[getter]
    fn aws_key_secret(&self) -> Option<String> {
        match &self.aws_key_secret.0 {
            ROption::RSome(x) => Some(x.to_string()),
            ROption::RNone => None,
        }
    }
    #[getter]
    fn aws_key_env_var(&self) -> Option<String> {
        match &self.aws_key_env_var.0 {
            ROption::RSome(x) => Some(x.to_string()),
            ROption::RNone => None,
        }
    }
    #[getter]
    fn aws_key_secret_env_var(&self) -> Option<String> {
        match &self.aws_key_secret_env_var.0 {
            ROption::RSome(x) => Some(x.to_string()),
            ROption::RNone => None,
        }
    }
    #[getter]
    pub fn region(&self) -> String {
        self.region.to_string()
    }
    #[getter]
    pub fn project_name(&self) -> String {
        self.project_name.to_string()
    }
}
