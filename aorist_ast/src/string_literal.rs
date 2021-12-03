use crate::{AncestorRecord, AST};
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_extendr_api::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::hash::Hash;
use aorist_primitives::AString;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct StringLiteral {
    value: AString,
    is_sql: bool,
    ancestors: Option<Vec<AncestorRecord>>,
}

impl StringLiteral {
    pub fn new(value: AString, is_sql: bool) -> Self {
        assert!(value.len() > 0 || !is_sql);
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
    pub fn pretty_sql_value(&self, depth: usize) -> AString {
        assert!(self.is_sql);
        let splits: Vec<String> = self
            .value
            .as_str()
            .to_string()
            .split("\n")
            .filter(|x| x.len() > 0)
            .map(|x| x.to_string())
            .collect();
        if splits.len() == 0 {
            panic!("Cannot pretify SQL value: {}", self.value);
        }
        if splits.len() == 1 {
            return AString::new(&splits.into_iter().next().unwrap());
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
        let out = format!("\n{}\n{}", pretty_splits.join("\n"), offset,);
        AString::new(&out)
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
        ast_module.getattr("Constant")?.call1((value.as_str(),))
    }

    pub fn to_r_ast_node(&self, depth: usize) -> Robj {
        let value = match self.is_sql {
            false => self.value.clone(),
            true => self.pretty_sql_value(depth),
        };
        Robj::from(vec![value.as_str()])
    }

    pub fn new_wrapped(value: AString, is_sql: bool) -> RArc<RRwLock<Self>> {
        RArc::new(RRwLock::new(Self::new(value, is_sql)))
    }
    pub fn value(&self) -> AString {
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
