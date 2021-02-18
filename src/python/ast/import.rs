use crate::python::ast::{SimpleIdentifier, AST};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Import {
    ModuleImport(String, Option<String>),
    FromImport(String, String, Option<String>),
}
impl Import {
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
    ) -> PyResult<&'a PyAny> {
        match &self {
            Self::ModuleImport(ref module, ref alias) => {
                let alias_list = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                    module.clone(),
                                ))
                                .to_python_ast_node(py, ast_module)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module)?,
                            ),
                        )?,
                        None => {
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(module.clone()))
                                .to_python_ast_node(py, ast_module)?
                        }
                    }],
                );
                ast_module.call1("Import", (alias_list,))
            }
            Self::FromImport(ref module, ref name, ref alias) => {
                let alias = PyList::new(
                    py,
                    vec![match alias {
                        Some(ref x) => ast_module.call1(
                            "alias",
                            (
                                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                                    .to_python_ast_node(py, ast_module)?,
                                (AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(x.clone())))
                                    .to_python_ast_node(py, ast_module)?,
                            ),
                        )?,
                        None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name.clone()))
                            .to_python_ast_node(py, ast_module)?,
                    }],
                );
                ast_module.call1("ImportFrom", (module, alias.as_ref(), 0))
            }
        }
    }
}
