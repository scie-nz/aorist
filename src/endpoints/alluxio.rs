#![allow(non_snake_case)]
use getset::{Getters, Setters};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Getters, Setters)]
pub struct AlluxioConfig {
    #[getset(get = "pub", set = "pub")]
    server: String,
    #[getset(get = "pub", set = "pub")]
    rpcPort: usize,
    #[getset(get = "pub", set = "pub")]
    apiPort: usize,
}

#[pymethods]
impl AlluxioConfig {
    #[new]
    #[args(rpcPort = "19999", apiPort = "39999")]
    fn new(server: String, rpcPort: usize, apiPort: usize) -> Self {
        Self {
            server,
            rpcPort,
            apiPort,
        }
    }
}
