use std::collections::HashMap;

pub struct Context {
    inner: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
}
