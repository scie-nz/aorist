use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use crate::{AST, BooleanLiteral, BigIntLiteral, StringLiteral};

pub fn extract_arg(arg: &PyAny) -> PyResult<AST> {
    if let Ok(extracted_val) = arg.extract::<String>() {
        Ok(AST::StringLiteral(StringLiteral::new_wrapped(extracted_val, false)))
    } else if let Ok(extracted_val) = arg.extract::<bool>() {
        Ok(AST::BooleanLiteral(BooleanLiteral::new_wrapped(extracted_val)))
    } else if let Ok(extracted_val) = arg.extract::<i64>() {
        Ok(AST::BigIntLiteral(BigIntLiteral::new_wrapped(extracted_val)))
    } else {
        Err(PyValueError::new_err("Object can be either string, boolean, or int"))
    }
}
