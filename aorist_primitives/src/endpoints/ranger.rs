#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
pub struct RangerConfig {
    server: String,
    port: usize,
    user: String,
    password: String,
}

#[cfg_attr(feature = "python", pymethods)]
impl RangerConfig {
    #[new]
    fn new(
        server: String,
        port: usize,
        user: String,
        password: String,
    ) -> Self {
        Self { server, port, user, password}
    }
}
