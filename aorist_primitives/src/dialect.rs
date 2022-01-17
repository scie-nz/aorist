use abi_stable::{std_types::*, StableAbi};
use crate::concept::AString;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use std::collections::BTreeSet;

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct Python {
    pip_requirements: RVec<AString>,
}
#[cfg(feature = "python")]
#[pymethods]
impl Python {
    #[new]
    pub fn new(pip_requirements: Vec<&str>) -> Self {
        Self {
            pip_requirements: pip_requirements.into_iter().map(|x| x.into()).collect(),
        }
    }
    pub fn get_pip_requirements(&self) -> BTreeSet<AString> {
        self.pip_requirements.clone().into_iter().collect()
    }
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct R {}

#[cfg(feature = "python")]
#[pymethods]
impl R {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct Bash {}
#[cfg(feature = "python")]
#[pymethods]
impl Bash {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[repr(C)]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub struct Presto {}
#[cfg(feature = "python")]
#[pymethods]
impl Presto {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[repr(C)]
#[cfg_attr(feature = "python", derive(FromPyObject))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, StableAbi)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
    Presto(Presto),
}

#[cfg(feature = "python")]
pub fn dialects_module(
    _py: pyo3::prelude::Python,
    m: &pyo3::prelude::PyModule,
) -> pyo3::prelude::PyResult<()> {
    m.add_class::<Python>()?;
    m.add_class::<Bash>()?;
    m.add_class::<Presto>()?;
    m.add_class::<R>()?;
    Ok(())
}
