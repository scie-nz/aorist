#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use crate::concept::{AString};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct GiteaConfig {
    server: AString,
    port: usize,
    token: AString,
}
#[cfg(feature = "python")]
#[pymethods]
impl GiteaConfig {
    #[new]
    fn new(server: String, port: usize, token: String) -> Self {
        GiteaConfig {
            server: server.as_str().into(),
            port,
            token: token.as_str().into(),
        }
    }
}
