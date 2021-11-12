use crate::{BigIntLiteral, BooleanLiteral, FloatLiteral, List, None, StringLiteral, Tuple, AST, Dict, SimpleIdentifier, Call, Attribute};
use aorist_primitives::Context;
use linked_hash_map::LinkedHashMap;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple, PyDict, PyType};

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
    } else if let Ok(extracted_dict) = arg.downcast::<PyDict>() {
        let m = extracted_dict
            .iter()
            .map(|(k, v)| (
                match k.extract::<String>() {
                    Ok(key) => key.clone(),
                    Err(err) => panic!("Dictionary keys should be string. Got {:?} instead:\n{:?}", k, err),
                }, 
                match extract_arg(v) {
                    Ok(val) => val,
                    Err(err) => panic!("Problem when extracting value for key: {:?}:\n{:?}", k, err),
                }
            ))
            .collect::<LinkedHashMap<String, AST>>();
        Ok(AST::Dict(Dict::new_wrapped(m)))
    } else if arg.is_none() {
        Ok(AST::None(crate::None::new_wrapped()))
    } else if let Ok(extracted_type) = arg.downcast::<PyType>() {
        Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(extracted_type.name()?.to_string())))
    } else {
        let class = arg.getattr("__class__")?;
        let module = class.getattr("__module__")?.extract::<String>()?;
        let name = class.getattr("__name__")?.extract::<&str>()?;
        if module == "ast" {
            match name {
               "Call" => {
                    let func = extract_arg(arg.getattr("func")?)?;
                    let args = match extract_arg(arg.getattr("args")?)? {
                        AST::List(x) => x.read().unwrap().elems().clone(),
                        AST::None(x) => Vec::new(),
                        _ => panic!("args field of call should be list or none"),
                    };
                    let keywords = arg.getattr("keywords")?.downcast::<PyList>()?;
                    let kwmap = keywords.into_iter().map(
                        |x| (x.getattr("arg").unwrap(), x.getattr("value").unwrap())
                    ).map(
                        |x| (x.0.extract::<String>().unwrap(), extract_arg(x.1).unwrap())
                    ).collect::<LinkedHashMap<String, AST>>();
                    Ok(AST::Call(Call::new_wrapped(func, args, kwmap)))
              },
              "Attribute" => {
                    let value = extract_arg(arg.getattr("value")?)?;
                    let attr = arg.getattr("attr")?.extract::<String>()?;
                    Ok(AST::Attribute(Attribute::new_wrapped(value, attr, false)))
              },
              "Name" => {
                    let id = arg.getattr("id")?.extract::<String>()?;
                    Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(id)))
              },
              "Constant" => {
                    let value = arg.getattr("value")?;
                    Ok(extract_arg(value)?)
              },
              _ => panic!("Not sure what to do with ast.{}", name)
          }
        } else {
            Err(PyValueError::new_err(
                "Object can be either string, boolean, int, a list, a tuple, or a type",
            ))
        }
    }
}
pub fn extract_arg_with_context(arg: &PyAny, context: &mut Context, constraint_name: String) -> PyResult<AST> {
    if let Ok((py_any, extracted_context)) = arg.extract::<(&PyAny, Context)>() {
        context.insert(&extracted_context, constraint_name);
        extract_arg(py_any)
    } else {
        Err(PyValueError::new_err(
            "Lambdas containing context should return a tuple of the form (x, context), where x string, bool or int."
        ))
    }
}
