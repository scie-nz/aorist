#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::collections::HashMap;
use tracing::debug;

#[cfg_attr(feature = "python", pyclass)]
#[derive(Clone)]
pub struct Context {
    inner: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    pub fn insert(&mut self, other: &Self) {
        for (k, v) in other.inner.iter() {
            if let Some(existing_val) = self.inner.get(k) {
                if existing_val != v {
                    panic!(
                        "Tried to insert non-identical value for {}: {} != {}.",
                        k, v, existing_val
                    );
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
        self.inner.insert(key.clone(), value.clone());
        debug!("Captured ({}, {})", &key, &value);
        value
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).and_then(|x| Some(x.clone()))
    }
}
