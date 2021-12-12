use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::code::Import;
use aorist_ast::{SimpleIdentifier, AST};
use aorist_primitives::AString;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PythonImport {
    PythonModuleImport(AString, AOption<AString>),
    PythonFromImport(AString, AString, AOption<AString>),
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
                        Some(ref x) => ast_module.getattr("alias")?.call1((
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(module.clone()))
                                .to_python_ast_node(py, ast_module, depth)?,
                            (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                .to_python_ast_node(py, ast_module, depth)?,
                        ))?,
                        None => {
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(module.clone()))
                                .to_python_ast_node(py, ast_module, depth)?
                        }
                    }],
                );
                ast_module.getattr("Import")?.call1((alias_list,))
            }
            Self::PythonFromImport(ref module, ref name, ref alias) => {
                let alias = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.getattr("alias")?.call1((
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                                .to_python_ast_node(py, ast_module, depth)?,
                            (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                .to_python_ast_node(py, ast_module, depth)?,
                        ))?,
                        None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                            .to_python_ast_node(py, ast_module, depth)?,
                    }],
                );
                ast_module
                    .getattr("ImportFrom")?
                    .call1((module.as_str(), alias, 0))
            }
        }
    }
}
