mod ast;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyString, PyTuple};
use std::collections::{BTreeSet, HashMap};

pub use ast::{
    Add, Assignment, Attribute, BashPythonTask, BigIntLiteral, BinOp, BooleanLiteral, Call,
    ConstantPythonTask, Dict, Expression, ForLoop, Formatted, Import, ImportNode, List,
    NativePythonTask, None, ParameterTuple, ParameterTupleDedupKey, Preamble, PrestoPythonTask,
    RPythonTask, SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
/// Wrapper type for stuff that gets passed around when building Python
/// statements:
/// - A vector of AST objects (main statements),
/// - A set of Preambles (which have their own imports attached)
/// - A set of imports corresponding to the dialect used.
/// - A comment string
pub type PythonStatementInput = (
    Vec<AST>,
    LinkedHashSet<Preamble>,
    BTreeSet<Import>,
    String,
    Option<String>,
    Option<String>,
);

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
