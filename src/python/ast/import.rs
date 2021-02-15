use crate::python::ast::SimpleIdentifier;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::{Hash};

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Import {
    ModuleImport(String),
    FromImport(String, String),
}
impl Import {
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::ModuleImport(ref module) => {
                let names = PyList::new(
                    py,
                    vec![SimpleIdentifier::new(module.clone()).to_python_ast_node(py, ast_module)?],
                );
                ast_module.call1("Import", (names.as_ref(),))
            }
            Self::FromImport(ref module, ref name) => {
                let names = PyList::new(
                    py,
                    vec![SimpleIdentifier::new(name.clone()).to_python_ast_node(py, ast_module)?],
                );
                ast_module.call1("ImportFrom", (module, names.as_ref(), 0))
            }
        }
    }
}

