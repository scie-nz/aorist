#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl RangerConfig {
    #[new]
    fn new(server: String, port: usize, user: String, password: String) -> Self {
        Self {
            server,
            port,
            user,
            password,
        }
    }
}
