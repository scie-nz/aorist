use std::collections::HashMap;

pub struct Context {
    inner: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }
    pub fn insert(&mut self, other: &Self) {
        for (k, v) in other.inner.iter() {
            if let Some(existing_val) = self.inner.get(k) {
                if existing_val != v {
                    panic!("Tried to insert non-identical value for {}: {} != {}.", k, v, existing_val);
                }
            } else {
                self.inner.insert(k.clone(), v.clone());
            }
        }
    }
}
