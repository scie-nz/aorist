#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash)]
pub struct LINZAPIConfig {
    pub koordinates_token: String,
    pub pii_allowed: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl LINZAPIConfig {
    #[new]
    fn new(
        koordinates_token: String,
        pii_allowed: bool,
    ) -> Self {
        LINZAPIConfig {
            koordinates_token,
            pii_allowed,
        }
    }
    #[getter]
    pub fn koordinates_token(&self) -> String {
        self.koordinates_token.clone()
    }
    #[getter]
    pub fn pii_allowed(&self) -> bool {
        self.pii_allowed
    }
}
