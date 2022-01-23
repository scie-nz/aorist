use aorist_util::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct AlluxioConfig {
    pub server: AString,
    pub server_cli: AString,
    pub rpc_port: usize,
    pub api_port: usize,
    pub directory: AString,
}

#[cfg(feature = "python")]
#[pymethods]
impl AlluxioConfig {
    #[new]
    fn new(
        server: AString,
        server_cli: AString,
        rpc_port: usize,
        api_port: usize,
        directory: AString,
    ) -> Self {
        AlluxioConfig {
            server: server,
            server_cli: server_cli,
            rpc_port,
            api_port,
            directory: directory,
        }
    }
}
