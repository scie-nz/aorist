#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
pub struct AlluxioConfig {
    pub server: String,
    pub server_cli: String,
    pub rpc_port: usize,
    pub api_port: usize,
    pub directory: String,
}
#[cfg_attr(feature = "python", pymethods)]
impl AlluxioConfig {
    #[new]
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
