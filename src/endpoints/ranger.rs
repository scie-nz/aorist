use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

#[pymethods]
impl RangerConfig {
    #[new]
    #[args(server="\"localhost\".to_string()", port="30800", user="\"admin\".to_string()")]
    fn new(server: String, port: usize, user: String, password: String) -> Self {
        Self {
            server, port, user, password,
        }
    }
}
