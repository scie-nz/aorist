#![allow(non_snake_case)]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct PrestoConfig {
    server: String,
    httpPort: usize,
}

#[pymethods]
impl PrestoConfig {
    #[new]
    #[args(httpPort = "8080")]
    fn new(server: String, httpPort: usize) -> Self {
        Self { server, httpPort }
    }
}
