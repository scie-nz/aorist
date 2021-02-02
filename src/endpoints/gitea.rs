use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct GiteaConfig {
    server: String,
    port: usize,
    token: String,
}
