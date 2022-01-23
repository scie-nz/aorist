use aorist_util::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct RangerConfig {
    server: AString,
    port: usize,
    user: AString,
    password: AString,
}

#[cfg(feature = "python")]
#[pymethods]
impl RangerConfig {
    #[new]
    fn new(server: String, port: usize, user: String, password: String) -> Self {
        Self {
            server: server.as_str().into(),
            port,
            user: user.as_str().into(),
            password: password.as_str().into(),
        }
    }
}
