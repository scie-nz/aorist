#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};
#[cfg_attr(feature = "python", pyclass(dict))]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    pub rpc_port: usize,
    pub api_port: usize,
    pub directory: String,
}
#[cfg_attr(feature = "python", pymethods)]
impl AlluxioConfig {
    #[cfg_attr(feature = "python", new)]
    fn new(
        server: String,
        server_cli: String,
        rpc_port: usize,
        api_port: usize,
        directory: String,
    ) -> Self {
        AlluxioConfig { server, server_cli, rpc_port, api_port, directory }
    }
}
