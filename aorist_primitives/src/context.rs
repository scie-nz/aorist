use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::HashMap;
use tracing::debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ContextStoredValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Vec<Box<ContextStoredValue>>),
}

impl std::fmt::Display for ContextStoredValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextStoredValue::String(x) => x.fmt(f),
            ContextStoredValue::Integer(x) => x.fmt(f),
            ContextStoredValue::Boolean(x) => x.fmt(f),
            ContextStoredValue::List(x) => x.iter().map(|y| y.fmt(f)).collect(),
        }
    }
}

#[cfg(feature = "python")]
impl ContextStoredValue {
    pub fn string(&self) -> PyResult<String> {
        match self {
            ContextStoredValue::String(x) => Ok(x.clone()),
            _ => Err(PyValueError::new_err("value is not string")),
        }
    }
    pub fn integer(&self) -> PyResult<i64> {
        match self {
            ContextStoredValue::Integer(x) => Ok(*x),
            _ => Err(PyValueError::new_err("value is not integer")),
        }
    }
    pub fn boolean(&self) -> PyResult<bool> {
        match self {
            ContextStoredValue::Boolean(x) => Ok(*x),
            _ => Err(PyValueError::new_err("value is not boolean")),
        }
    }
}

#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone)]
pub struct Context {
    inner: HashMap<String, ContextStoredValue>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, other: &Self) {
        for (k, v) in other.inner.iter() {
            let existing: Option<_> = self.inner.get(k).and_then(|x| Some(x.clone()));
            if let Some(existing_val) = existing {
                if existing_val != *v {
                    self.inner.insert(k.clone(), v.clone());
                    //panic!("Key {} already populated", k);
                    //self.inner
                    //    .insert(k.clone(), format!("{};{}", existing_val, v).to_string());
                }
            } else {
                debug!("Inserted from dependent constraint ({}, {})", &k, &v);
                self.inner.insert(k.clone(), v.clone());
            }
        }
    }
}

#[cfg_attr(feature = "python", pymethods)]
impl Context {
    pub fn capture(&mut self, key: String, value: String) -> String {
        self.inner
            .insert(key.clone(), ContextStoredValue::String(value.clone()));
        debug!("Captured ({}, {})", &key, &value);
        value
    }
    pub fn capture_int(&mut self, key: String, value: i64) -> i64 {
        self.inner
            .insert(key.clone(), ContextStoredValue::Integer(value));
        debug!("Captured ({}, {})", &key, &value);
        value
    }
    pub fn capture_bool(&mut self, key: String, value: bool) -> bool {
        self.inner
            .insert(key.clone(), ContextStoredValue::Boolean(value));
        debug!("Captured ({}, {})", &key, &value);
        value
    }
    pub fn get(&self, key: String) -> PyResult<String> {
        match self.inner.get(&key) {
            Some(x) => x.string(),
            None => Err(PyValueError::new_err(format!(
                "Could not find key {} in context",
                key
            ))),
        }
    }
    pub fn get_int(&self, key: String) -> PyResult<i64> {
        match self.inner.get(&key) {
            Some(x) => x.integer(),
            None => Err(PyValueError::new_err(format!(
                "Could not find key {} in context",
                key
            ))),
        }
    }
    pub fn get_bool(&self, key: String) -> PyResult<bool> {
        match self.inner.get(&key) {
            Some(x) => x.boolean(),
            None => Err(PyValueError::new_err(format!(
                "Could not find key {} in context",
                key
            ))),
        }
    }
}
