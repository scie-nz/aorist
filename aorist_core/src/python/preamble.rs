use crate::code::Preamble;
use crate::python::PythonImport;
use aorist_ast::FunctionDef;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule, PyString, PyTuple};
use std::hash::Hash;
use tracing::debug;

pub trait TPythonPreamble {
    fn to_python_ast_nodes<'b>(
        &self,
        py: Python<'b>,
        _ast_module: &'b PyModule,
        _depth: usize,
    ) -> Vec<&'b PyAny> {
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
        debug!("Import body: {}", self.get_body());
        let out: &PyList = helpers.getattr("to_nodes").unwrap()
            .call1((self.get_body(),))
            .unwrap()
            .downcast()
            .unwrap();

        out.into_iter().collect()
    }
    fn get_body(&self) -> String;
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum PythonPreamble {
    NativePythonPreamble(NativePythonPreamble),
    PythonStatementsPreamble(PythonStatementsPreamble),
    RPythonPreamble(RPythonPreamble),
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct NativePythonPreamble {
    pub imports: Vec<PythonImport>,
    pub from_imports: Vec<PythonImport>,
    pub body: String,
}
#[derive(Clone, PartialEq, Hash, Eq)]
pub struct RPythonPreamble {
    pub body: String,
}
#[derive(Clone, PartialEq, Hash, Eq)]
pub struct PythonStatementsPreamble {
    pub function: FunctionDef,
    pub imports: Vec<PythonImport>,
}
impl Preamble for PythonStatementsPreamble {
    type ImportType = PythonImport;
    fn get_imports(&self) -> Vec<Self::ImportType> {
        self.imports.clone()
    }
}
impl Preamble for RPythonPreamble {
    type ImportType = PythonImport;
    fn get_imports(&self) -> Vec<Self::ImportType> {
        vec![PythonImport::PythonModuleImport(
            "rpy2.objects".to_string(),
            Some("robjects".to_string()),
        )]
    }
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
impl TPythonPreamble for RPythonPreamble {
    fn get_body(&self) -> String {
        format!(
            "robjects.r(\"\"\"{}\"\"\")",
            self.body.clone().replace("'", "\\'")
        )
        .to_string()
    }
}
impl TPythonPreamble for NativePythonPreamble {
    fn get_body(&self) -> String {
        self.body.clone()
    }
}
impl Preamble for PythonPreamble {
    type ImportType = PythonImport;
    fn get_imports(&self) -> Vec<PythonImport> {
        match &self {
            PythonPreamble::NativePythonPreamble(x) => x.get_imports(),
            PythonPreamble::RPythonPreamble(x) => x.get_imports(),
            PythonPreamble::PythonStatementsPreamble(x) => x.get_imports(),
        }
    }
}
impl PythonPreamble {
    pub fn to_python_ast_nodes<'b>(
        &self,
        py: Python<'b>,
        ast_module: &'b PyModule,
        depth: usize,
    ) -> Vec<&'b PyAny> {
        match &self {
            PythonPreamble::NativePythonPreamble(x) => x.to_python_ast_nodes(py, ast_module, depth),
            PythonPreamble::RPythonPreamble(x) => x.to_python_ast_nodes(py, ast_module, depth),
            PythonPreamble::PythonStatementsPreamble(x) => vec![x
                .function
                .to_python_ast_node(py, ast_module, depth)
                .unwrap()],
        }
    }
}
impl RPythonPreamble {
    pub fn new(body: String) -> PyResult<Self> {
        Ok(Self { body })
    }
}
impl NativePythonPreamble {
    pub fn new(body: String) -> PyResult<Self> {
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
            .getattr("build_preamble")?
            .call1((body,))?
            .downcast()
            .unwrap();

        let imports_list: &PyList = tpl.get_item(0).extract()?;
        let imports: Vec<PythonImport> = imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract().unwrap();
                let name: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()?
                    .to_str()?
                    .to_string();
                let alias = tpl.get_item(1);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()?
                            .to_str()?
                            .to_string(),
                    ),
                };
                Ok(PythonImport::PythonModuleImport(name, asname))
            })
            .collect::<PyResult<_>>()?;

        let from_imports_list: &PyList = tpl.get_item(1).extract()?;
        let from_imports: Vec<PythonImport> = from_imports_list
            .iter()
            .map(|x| {
                let tpl: &PyTuple = x.extract()?;
                let module: String = tpl
                    .get_item(0)
                    .extract::<&PyString>()?
                    .to_str()?
                    .to_string();
                let name: String = tpl
                    .get_item(1)
                    .extract::<&PyString>()?
                    .to_str()?
                    .to_string();
                let alias = tpl.get_item(2);
                let asname: Option<String> = match alias.is_none() {
                    true => None,
                    false => Some(
                        alias
                            .extract::<&PyString>()?
                            .to_str()?
                            .to_string(),
                    ),
                };
                Ok(PythonImport::PythonFromImport(module, name, asname))
            })
            .collect::<PyResult<_>>()?;

        let body_no_imports: &PyList = tpl.get_item(2).extract()?;
        Ok(Self {
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
        })
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
