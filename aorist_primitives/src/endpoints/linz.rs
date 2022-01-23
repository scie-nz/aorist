use aorist_util::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(PartialEq, Deserialize, Serialize, Debug, Clone, Hash, abi_stable::StableAbi)]
pub struct LINZAPIConfig {
    pub koordinates_token: AString,
    pub pii_allowed: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl LINZAPIConfig {
    #[new]
    fn new(koordinates_token: String, pii_allowed: bool) -> Self {
        LINZAPIConfig {
            koordinates_token: koordinates_token.as_str().into(),
            pii_allowed,
        }
    }
    #[getter]
    pub fn koordinates_token(&self) -> String {
        self.koordinates_token.to_string()
    }
    #[getter]
    pub fn pii_allowed(&self) -> bool {
        self.pii_allowed
    }
}
