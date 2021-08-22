use crate::{AncestorRecord, AST};
use aorist_extendr_api::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct StringLiteral {
    value: String,
    is_sql: bool,
    ancestors: Option<Vec<AncestorRecord>>,
}

impl StringLiteral {
    pub fn new(value: String, is_sql: bool) -> Self {
        Self {
            value,
            is_sql,
            ancestors: None,
        }
    }
    pub fn set_ancestors(&mut self, ancestors: Vec<AncestorRecord>) {
        assert!(self.ancestors.is_none());
        self.ancestors = Some(ancestors);
    }
    pub fn get_ancestors(&self) -> Option<Vec<AncestorRecord>> {
        self.ancestors.clone()
    }
    pub fn clone_without_ancestors(&self) -> Self {
        Self {
            value: self.value.clone(),
            is_sql: self.is_sql,
            ancestors: None,
        }
    }
    pub fn as_sql_string(&self) -> Self {
        Self {
            value: self.value.clone(),
            is_sql: true,
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
        ast_module.getattr("Constant")?.call1((&value,))
    }

    pub fn to_r_ast_node(&self, depth: usize) -> Robj {
        let value = match self.is_sql {
            false => self.value.clone(),
            true => self.pretty_sql_value(depth),
        };
        Robj::from(vec![&*value])
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
    pub fn is_multiline(&self) -> bool {
        self.value.contains('\n')
    }
    pub fn get_direct_descendants(&self) -> Vec<AST> {
        Vec::new()
    }
    pub fn optimize_fields(&self) {}
}
