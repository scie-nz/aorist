#![allow(non_snake_case)]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct PrestoConfig {
    server: String,
    httpPort: usize,
}
