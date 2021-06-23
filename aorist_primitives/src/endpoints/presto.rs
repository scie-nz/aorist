#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
pub struct PrestoConfig {
    pub server: String,
    pub http_port: usize,
    pub user: String,
}
#[cfg_attr(feature = "python", pymethods)]
impl PrestoConfig {
    #[new]
    fn new(
        server: String,
        http_port: usize,
        user: String,
    ) -> Self {
        PrestoConfig { server, http_port, user }
    }
}
