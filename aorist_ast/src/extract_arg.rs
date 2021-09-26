use crate::{BigIntLiteral, BooleanLiteral, FloatLiteral, List, None, StringLiteral, Tuple, AST};
use aorist_primitives::Context;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

pub fn extract_arg(arg: &PyAny) -> PyResult<AST> {
    if arg.is_none() {
        Ok(AST::None(None::new_wrapped()))
    } else if let Ok(extracted_val) = arg.extract::<String>() {
        Ok(AST::StringLiteral(StringLiteral::new_wrapped(
            extracted_val,
            false,
        )))
    } else if let Ok(extracted_val) = arg.extract::<bool>() {
        Ok(AST::BooleanLiteral(BooleanLiteral::new_wrapped(
            extracted_val,
        )))
    } else if let Ok(extracted_val) = arg.extract::<i64>() {
        Ok(AST::BigIntLiteral(BigIntLiteral::new_wrapped(
            extracted_val,
        )))
    } else if let Ok(extracted_list) = arg.downcast::<PyList>() {
        let v = extracted_list
            .iter()
            .map(|x| extract_arg(x))
            .collect::<PyResult<Vec<AST>>>()?;
        Ok(AST::List(List::new_wrapped(v, false)))
    } else if let Ok(extracted_val) = arg.downcast::<pyo3::types::PyFloat>() {
        Ok(AST::FloatLiteral(FloatLiteral::new_wrapped(
            aorist_attributes::FloatValue::from_f64(extracted_val.value()),
        )))
    } else if let Ok(extracted_tuple) = arg.downcast::<PyTuple>() {
        let v = extracted_tuple
            .iter()
            .map(|x| extract_arg(x))
            .collect::<PyResult<Vec<AST>>>()?;
        Ok(AST::Tuple(Tuple::new_wrapped(v, false)))
    } else {
        Err(PyValueError::new_err(
            "Object can be either string, boolean, int, or a list",
        ))
    }
}
pub fn extract_arg_with_context(arg: &PyAny, context: &mut Context) -> PyResult<AST> {
    if let Ok((extracted_val, extracted_context)) = arg.extract::<(String, Context)>() {
        context.insert(&extracted_context);
        Ok(AST::StringLiteral(StringLiteral::new_wrapped(
            extracted_val,
            false,
        )))
    } else if let Ok((extracted_val, extracted_context)) = arg.extract::<(bool, Context)>() {
        context.insert(&extracted_context);
        Ok(AST::BooleanLiteral(BooleanLiteral::new_wrapped(
            extracted_val,
        )))
    } else if let Ok((extracted_val, extracted_context)) = arg.extract::<(i64, Context)>() {
        context.insert(&extracted_context);
        Ok(AST::BigIntLiteral(BigIntLiteral::new_wrapped(
            extracted_val,
        )))
    } else {
        Err(PyValueError::new_err(
            "Object can be either string, boolean, or int",
        ))
    }
}
