use crate::concept::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct PostgresConfig {
    pub server: AString,
    pub port: usize,
    pub username: AString,
    pub password: AString,
}
#[cfg(feature = "python")]
#[pymethods]
impl PostgresConfig {
    #[new]
    fn new(server: String, port: usize, username: String, password: String) -> Self {
        PostgresConfig {
            server: server.as_str().into(),
            port,
            username: username.as_str().into(),
            password: password.as_str().into(),
        }
    }
}
