#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::BTreeSet;

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Python {
    pip_requirements: BTreeSet<String>,
}
#[cfg(feature = "python")]
#[pymethods]
impl Python {
    #[new]
    pub fn new(pip_requirements: Vec<String>) -> Self {
        Self {
            pip_requirements: pip_requirements.into_iter().collect(),
        }
    }
    pub fn get_pip_requirements(&self) -> BTreeSet<String> {
        self.pip_requirements.clone()
    }
}
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct R {}

#[cfg(feature = "python")]
#[pymethods]
impl R {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bash {}
#[cfg(feature = "python")]
#[pymethods]
impl Bash {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Presto {}
#[cfg(feature = "python")]
#[pymethods]
impl Presto {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(feature = "python", derive(FromPyObject))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
    Presto(Presto),
}

#[cfg(feature = "python")]
pub fn dialects_module(_py: pyo3::prelude::Python, m: &pyo3::prelude::PyModule) -> pyo3::prelude::PyResult<()> {
    m.add_class::<Python>()?;
    m.add_class::<Bash>()?;
    m.add_class::<Presto>()?;
    m.add_class::<R>()?;
    Ok(())
}
