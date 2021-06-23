#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
pub struct PostgresConfig {
    pub server: String,
    pub port: usize,
    pub username: String,
    pub password: String,
}
#[cfg_attr(feature = "python", pymethods)]
impl PostgresConfig {
    #[new]
    fn new(
        server: String,
        port: usize,
        username: String,
        password: String,
    ) -> Self {
        PostgresConfig { server, port, username, password }
    }
}
