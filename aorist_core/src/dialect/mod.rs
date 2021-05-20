use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Python {
    pip_requirements: BTreeSet<String>,
}
impl Python {
    pub fn new(pip_requirements: Vec<String>) -> Self {
        Self {
            pip_requirements: pip_requirements.into_iter().collect(),
        }
    }
    pub fn get_pip_requirements(&self) -> BTreeSet<String> {
        self.pip_requirements.clone()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct R {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bash {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Presto {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Dialect {
    Python(Python),
    R(R),
    Bash(Bash),
    Presto(Presto),
}
