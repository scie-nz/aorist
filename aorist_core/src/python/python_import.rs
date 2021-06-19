use crate::code::Import;
use aorist_ast::{SimpleIdentifier, AST};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PythonImport {
    PythonModuleImport(String, Option<String>),
    PythonFromImport(String, String, Option<String>),
}
impl Import for PythonImport {}

impl PythonImport {
    pub fn to_string(&self) -> String {
        match &self {
            Self::PythonModuleImport(ref module, Some(ref alias)) => {
                format!("import {} as {}", module, alias).to_string()
            }
            Self::PythonModuleImport(ref module, None) => format!("import {}", module).to_string(),
            Self::PythonFromImport(ref module, ref name, Some(ref alias)) => {
                format!("from {} import {} as {}", module, name, alias).to_string()
            }
            Self::PythonFromImport(ref module, ref name, None) => {
                format!("from {} import {}", module, name).to_string()
            }
        }
    }
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
        depth: usize,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::PythonModuleImport(ref module, ref alias) => {
                let alias_list = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                    module.clone(),
                                ))
                                .to_python_ast_node(py, ast_module, depth)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module, depth)?,
                            ),
                        )?,
                        None => {
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(module.clone()))
                                .to_python_ast_node(py, ast_module, depth)?
                        }
                    }],
                );
                ast_module.call1("Import", (alias_list,))
            }
            Self::PythonFromImport(ref module, ref name, ref alias) => {
                let alias = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                                    .to_python_ast_node(py, ast_module, depth)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module, depth)?,
                            ),
                        )?,
                        None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                            .to_python_ast_node(py, ast_module, depth)?,
                    }],
                );
                ast_module.call1("ImportFrom", (module, alias.as_ref(), 0))
            }
        }
    }
}
