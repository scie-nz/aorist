use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}

#[pymethods]
impl GiteaConfig {
    #[new]
    #[args(server = "\"localhost\".to_string()", port = "30807")]
    fn new(server: String, port: usize, token: String) -> Self {
        Self {
            server,
            port,
            token,
        }
    }
}
