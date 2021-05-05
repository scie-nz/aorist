mod ast;
mod code_block;
mod preamble;
mod python_import;
mod task;

use crate::flow::FlowBuilderInput;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyString, PyTuple};
use std::collections::{BTreeSet, HashMap};

pub use ast::{
    Add, Assignment, Attribute, BashPythonTask, BigIntLiteral, BinOp, BooleanLiteral, Call,
    ConstantPythonTask, Dict, Expression, ForLoop, Formatted, List, NativePythonTask, None,
    PrestoPythonTask, PythonImportNode, RPythonTask, SimpleIdentifier, StringLiteral, Subscript,
    Tuple, AST,
};
pub use code_block::PythonBasedCodeBlock;
pub use preamble::PythonPreamble;
pub use python_import::PythonImport;
pub use task::{ForLoopPythonBasedTask, PythonBasedTask, StandalonePythonBasedTask};

/// Wrapper type for stuff that gets passed around when building Python
/// statements:
/// - A vector of AST objects (main statements),
/// - A set of PythonPreambles (which have their own imports attached)
/// - A set of imports corresponding to the dialect used.
/// - A comment string
pub struct PythonFlowBuilderInput {
    statements: Vec<AST>,
    preambles: LinkedHashSet<PythonPreamble>,
    imports: BTreeSet<PythonImport>,
    constraint_name: String,
    constraint_title: Option<String>,
    constraint_body: Option<String>,
}
impl FlowBuilderInput for PythonFlowBuilderInput {

    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;

    fn new(
        statements: Vec<AST>,
        preambles: LinkedHashSet<PythonPreamble>,
        imports: BTreeSet<PythonImport>,
        constraint_name: String,
        constraint_title: Option<String>,
        constraint_body: Option<String>,
    ) -> Self {
        Self {
            statements,
            preambles,
            imports,
            constraint_name,
            constraint_title,
            constraint_body,
        }
    }
    fn get_statements(&self) -> Vec<AST> {
        self.statements.clone()
    }
    fn get_preambles(&self) -> LinkedHashSet<PythonPreamble> {
        self.preambles.clone()
    }
    fn get_imports(&self) -> BTreeSet<PythonImport> {
        self.imports.clone()
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn get_constraint_title(&self) -> Option<String> {
        self.constraint_title.clone()
    }
    fn get_constraint_body(&self) -> Option<String> {
        self.constraint_body.clone()
    }
}

pub fn format_code(code: String) -> PyResult<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let black: &PyModule = PyModule::import(py, "black").unwrap();
    let mut kwargs = HashMap::<&str, usize>::new();
    kwargs.insert("line_length", 80);
    let mode = black.call("FileMode", (), Some(kwargs.into_py_dict(py)))?;

    let py_code = PyString::new(py, &code);

    let mut kwargs = HashMap::<&str, &PyAny>::new();
    kwargs.insert("mode", mode);
    if let Ok(res) = black.call(
        "format_str",
        PyTuple::new(py, &[py_code]),
        Some(kwargs.into_py_dict(py)),
    ) {
        return res.extract();
    } else {
        panic!("Error formatting code block: \n{}\n---", py_code);
    }
}
