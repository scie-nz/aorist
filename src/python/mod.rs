mod ast;
mod import;
mod preamble;

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
pub use import::PythonImport;
pub use preamble::PythonPreamble;

/// Wrapper type for stuff that gets passed around when building Python
/// statements:
/// - A vector of AST objects (main statements),
/// - A set of PythonPreambles (which have their own imports attached)
/// - A set of imports corresponding to the dialect used.
/// - A comment string
pub type PythonStatementInput = (
    Vec<AST>,
    LinkedHashSet<PythonPreamble>,
    BTreeSet<PythonImport>,
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
