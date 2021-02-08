use crate::python::ast::ArgType;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::BTreeSet;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Hash, PartialEq, Eq)]
pub struct StringLiteral {
    value: String,
    // TODO: replace with LinkedHashMap<Uuid, BTreeSet>
    object_uuids: LinkedHashMap<Uuid, BTreeSet<Option<String>>>,
    owner: Option<ArgType>,
}

impl StringLiteral {
    fn new(value: String) -> Self {
        Self {
            value,
            object_uuids: LinkedHashMap::new(),
            owner: None,
        }
    }
    pub fn to_python_ast_node<'a>(
        &self,
        py: Python,
        ast_module: &'a PyModule,
    ) -> PyResult<&'a PyAny> {
        (|literal: &StringLiteral, _py: Python, ast_module: &'a PyModule| {
            ast_module.call1("Constant", (&literal.value,))
        })(self, py, ast_module)
    }
    pub fn new_wrapped(value: String) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(value)))
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn register_object(&mut self, uuid: Uuid, tag: Option<String>) {
        self.object_uuids
            .entry(uuid)
            .or_insert(BTreeSet::new())
            .insert(tag);
    }
    pub fn set_owner(&mut self, obj: ArgType) {
        self.owner = Some(obj);
    }
    pub fn get_object_uuids(&self) -> &LinkedHashMap<Uuid, BTreeSet<Option<String>>> {
        &self.object_uuids
    }
    pub fn is_multiline(&self) -> bool {
        self.value.contains('\n')
    }
    pub fn get_owner(&self) -> Option<ArgType> {
        self.owner.clone()
    }
    pub fn get_ultimate_owner(&self) -> Option<ArgType> {
        if self.get_owner().is_none() {
            return None;
        }
        let mut owner = self.get_owner().unwrap();
        while let Some(x) = owner.get_owner() {
            owner = x;
        }
        Some(owner.clone())
    }
    pub fn remove_owner(&mut self) {
        self.owner = None;
    }
    pub fn get_direct_descendants(&self) -> Vec<ArgType> {
        Vec::new()
    }
}
