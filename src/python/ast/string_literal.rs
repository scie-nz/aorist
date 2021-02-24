use crate::constraint_state::AncestorRecord;
use crate::python::ast::AST;
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
    is_sql: bool,
    // TODO: replace with LinkedHashMap<Uuid, BTreeSet>
    object_uuids: LinkedHashMap<Uuid, BTreeSet<Option<String>>>,
    ancestors: Option<Vec<AncestorRecord>>,
}

impl StringLiteral {
    pub fn new(value: String, is_sql: bool) -> Self {
        Self {
            value,
            is_sql,
            object_uuids: LinkedHashMap::new(),
            ancestors: None,
        }
    }
    pub fn set_ancestors(&mut self, ancestors: Vec<AncestorRecord>) {
        assert!(self.ancestors.is_none());
        self.ancestors = Some(ancestors);
    }
    pub fn as_sql_string(&self) -> Self {
        Self {
            value: self.value.clone(),
            is_sql: true,
            object_uuids: self.object_uuids.clone(),
            ancestors: self.ancestors.clone(),
        }
    }
    pub fn pretty_sql_value(&self, depth: usize) -> String {
        assert!(self.is_sql);
        let splits: Vec<String> = self
            .value
            .clone()
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| x.to_string())
            .collect();
        assert!(splits.len() > 0);
        if splits.len() == 1 {
            return splits.into_iter().next().unwrap();
        }
        let min_num_leading_spaces = splits
            .iter()
            .map(|x| {
                for (i, c) in x.chars().enumerate() {
                    if c != ' ' {
                        return i;
                    }
                }
                return x.len();
            })
            .min()
            .unwrap();
        let offset = (0..(depth * 4)).map(|_| " ").collect::<String>();
        let pretty_splits = splits
            .into_iter()
            .map(|mut x| {
                let without_leading_spaces = x.split_off(min_num_leading_spaces);
                format!("{}{}", &offset, without_leading_spaces).to_string()
            })
            .collect::<Vec<String>>();
        format!("\n{}\n{}", pretty_splits.join("\n"), offset,).to_string()
    }
    pub fn to_python_ast_node<'a>(
        &self,
        _py: Python,
        ast_module: &'a PyModule,
        depth: usize,
    ) -> PyResult<&'a PyAny> {
        let value = match self.is_sql {
            false => self.value.clone(),
            true => self.pretty_sql_value(depth),
        };
        ast_module.call1("Constant", (&value,))
    }
    pub fn new_wrapped(value: String, is_sql: bool) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(value, is_sql)))
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
    pub fn get_object_uuids(&self) -> &LinkedHashMap<Uuid, BTreeSet<Option<String>>> {
        &self.object_uuids
    }
    pub fn is_multiline(&self) -> bool {
        self.value.contains('\n')
    }
    pub fn get_direct_descendants(&self) -> Vec<AST> {
        Vec::new()
    }
}
