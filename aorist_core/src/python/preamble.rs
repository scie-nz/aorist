use crate::code::Preamble;
use crate::python::PythonImport;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::hash::Hash;

pub trait TPythonPreamble {
    fn get_body_ast<'b>(&self, py: Python<'b>) -> Vec<&'b PyAny>;
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum PythonPreamble {
    NativePythonPreamble(NativePythonPreamble),
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct NativePythonPreamble {
    pub imports: Vec<PythonImport>,
    pub from_imports: Vec<PythonImport>,
    pub body: String,
}
impl Preamble for NativePythonPreamble {
    type ImportType = PythonImport;
    fn get_imports(&self) -> Vec<Self::ImportType> {
        self.imports
            .clone()
            .into_iter()
            .chain(self.from_imports.clone().into_iter())
            .collect()
    }
}
impl TPythonPreamble for NativePythonPreamble {
    fn get_body_ast<'b>(&self, py: Python<'b>) -> Vec<&'b PyAny> {
        let helpers = PyModule::from_code(
            py,
            r#"
import ast

def to_nodes(body):
    module = ast.parse(body)
    return module.body
        "#,
            "helpers.py",
            "helpers",
        )
        .unwrap();

        let out: &PyList = helpers
            .call1("to_nodes", (self.body.clone(),))
            .unwrap()
            .downcast()
            .unwrap();

        out.into_iter().collect()
    }
}
impl Preamble for PythonPreamble {
    type ImportType = PythonImport;
    fn get_imports(&self) -> Vec<Self::ImportType> {
        match &self {
            PythonPreamble::NativePythonPreamble(x) => x.get_imports(),
        }
    }
}
impl TPythonPreamble for PythonPreamble {
    fn get_body_ast<'b>(&self, py: Python<'b>) -> Vec<&'b PyAny> {
        match &self {
            PythonPreamble::NativePythonPreamble(x) => x.get_body_ast(py),
        }
    }
}
impl NativePythonPreamble {
    pub fn new(body: String) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let helpers = PyModule::from_code(
            py,
            r#"
import ast
import astor

def build_preamble(body):
    module = ast.parse(body)

    imports = []
    from_imports = []
    other = []

    for elem in module.body:
        if isinstance(elem, ast.Import):
            for name in elem.names:
                imports += [(name.name, name.asname)]
        elif isinstance(elem, ast.ImportFrom):
            for name in elem.names:
                from_imports += [(elem.module, name.name, name.asname)]
        else:
            other += [astor.to_source(elem)]

    return imports, from_imports, other
        "#,
            "helpers.py",
            "helpers",
        )
        .unwrap();

        let tpl: &PyTuple = helpers
            .call1("build_preamble", (body,))
            .unwrap()
            .downcast()
            .unwrap();

        let imports_list: &PyList = tpl.get_item(0).extract().unwrap();
        let imports: Vec<PythonImport> = imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let name: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(1);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                PythonImport::PythonModuleImport(name, asname)
            })
            .collect();

        let from_imports_list: &PyList = tpl.get_item(1).extract().unwrap();
        let from_imports: Vec<PythonImport> = from_imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let module: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let name: String = tpl
                    .get_item(1)
                    .extract::<&PyString>()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let alias = tpl.get_item(2);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                    ),
                };
                PythonImport::PythonFromImport(module, name, asname)
            })
            .collect();

        let body_no_imports: &PyList = tpl.get_item(2).extract().unwrap();
        Self {
            imports,
            from_imports,
            body: body_no_imports
                .iter()
                .map(|x| {
                    x.extract::<&PyString>()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join("\n"),
        }
    }
    pub fn to_string(&self) -> String {
        self.from_imports
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .chain(self.imports.clone().into_iter().map(|x| x.to_string()))
            .chain(vec![self.body.clone()].into_iter())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
