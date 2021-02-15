mod ast;
use linked_hash_set::LinkedHashSet;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyString, PyTuple};
use std::collections::{BTreeSet, HashMap};

pub use ast::{
    Assignment, Attribute, BashPythonTask, BigIntLiteral, BooleanLiteral, Call, ConstantPythonTask,
    Dict, Expression, ForLoop, Formatted, Import, List, NativePythonTask, ParameterTuple,
    ParameterTupleDedupKey, Preamble, PrestoPythonTask, PythonImport, PythonNone, RPythonTask,
    SimpleIdentifier, StringLiteral, Subscript, Tuple, AST,
};
pub type PythonStatementInput = (Vec<AST>, LinkedHashSet<String>, BTreeSet<Import>);

pub fn format_code(code: String) -> PyResult<String> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let black: &PyModule = PyModule::import(py, "black").unwrap();
    let mode = black.call0("Mode")?;

    let py_code = PyString::new(py, &code);

    let mut kwargs = HashMap::<&str, &PyAny>::new();
    kwargs.insert("mode", mode);
    black
        .call(
            "format_str",
            PyTuple::new(py, &[py_code]),
            Some(kwargs.into_py_dict(py)),
        )
        .unwrap()
        .extract()
}
