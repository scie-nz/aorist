mod ast;
mod code_block;
mod constraint_block;
mod preamble;
mod python_import;
mod task;

use crate::flow::FlowBuilderInput;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyString, PyTuple};
use std::collections::{BTreeSet, HashMap};

use aorist_ast::{
    Add, Assignment, Attribute, BigIntLiteral, BinOp, Call, Dict, ForLoop, Formatted, List,
    SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
pub use ast::{
    BashPythonTask, ConstantPythonTask, NativePythonTask, PrestoPythonTask, PythonTask, RPythonTask,
};
pub use code_block::PythonBasedCodeBlock;
pub use constraint_block::PythonBasedConstraintBlock;
pub use preamble::*;
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
impl PythonFlowBuilderInput {
    pub fn has_statements(&self) -> bool {
        self.statements.len() > 0
    }
    pub fn statements_only(
        statements: Vec<AST>,
        constraint_name: String,
        constraint_title: Option<String>,
        constraint_body: Option<String>,
    ) -> Self {
        Self::new(
            statements,
            LinkedHashSet::new(),
            BTreeSet::new(),
            constraint_name,
            constraint_title,
            constraint_body,
        )
    }
    pub fn to_python_ast_nodes<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
        depth: usize,
    ) -> PyResult<Vec<&'a PyAny>> {
        let mut v = Vec::new();
        for statement in self.get_statements() {
            v.push(statement.to_python_ast_node(py, ast_module, depth)?);
        }
        Ok(v)
    }
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
    let mode = black.getattr("FileMode")?.call((), Some(kwargs.into_py_dict(py)))?;

    let py_code = PyString::new(py, &code);

    let mut kwargs = HashMap::<&str, &PyAny>::new();
    kwargs.insert("mode", mode);
    if let Ok(res) = black.getattr(
        "format_str"
    )?.call(
        PyTuple::new(py, &[py_code]),
        Some(kwargs.into_py_dict(py)),
    ) {
        return res.extract();
    } else {
        panic!("Error formatting code block: \n{}\n---", py_code);
    }
}
