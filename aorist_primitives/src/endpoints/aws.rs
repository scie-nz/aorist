#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg_attr(feature = "python", pyclass)]
pub struct AWSConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub region: Option<String>,
    pub project_name: Option<String>,
}
#[cfg_attr(feature = "python", pymethods)]
impl AWSConfig {
    #[new]
    fn new(
        access_key_id: String,
        access_key_secret: String,
        region: Option<String>,
        project_name: Option<String>,
    ) -> Self {
        AWSConfig { access_key_id, access_key_secret, region, project_name }
    }
}
