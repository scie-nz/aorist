use crate::constraint_state::AncestorRecord;
use aorist_ast::{List, StringLiteral, AST};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParameterTuple {
    args: Vec<AST>,
    pub kwargs: LinkedHashMap<String, AST>,
}
pub type ParameterTupleDedupKey = (usize, Vec<String>);
impl ParameterTuple {
    pub fn set_ancestors(&self, ancestors: Vec<AncestorRecord>) {
        for arg in self.args.iter() {
            arg.set_ancestors(ancestors.clone());
        }
        for v in self.kwargs.values() {
            v.set_ancestors(ancestors.clone());
        }
    }
    pub fn populate_python_dict(&self, dict: &mut LinkedHashMap<String, AST>) {
        if self.args.len() > 0 {
            dict.insert(
                "args".to_string(),
                AST::List(List::new_wrapped(self.args.clone(), false)),
            );
        }
        for (key, val) in &self.kwargs {
            dict.insert(key.clone(), val.clone());
        }
    }
    pub fn get_dedup_key(&self) -> ParameterTupleDedupKey {
        (self.args.len(), self.kwargs.keys().cloned().collect())
    }
    pub fn new(
        _object_uuid: Uuid,
        args_v: Vec<String>,
        kwargs_v: LinkedHashMap<String, String>,
        is_sql: bool,
    ) -> Self {
        let args = args_v
            .into_iter()
            .map(|x| AST::StringLiteral(StringLiteral::new_wrapped(x, is_sql)))
            .collect::<Vec<_>>();
        let kwargs = kwargs_v
            .into_iter()
            .map(|(k, v)| (k, AST::StringLiteral(StringLiteral::new_wrapped(v, is_sql))))
            .collect::<LinkedHashMap<_, _>>();
        Self { args, kwargs }
    }
    pub fn get_args(&self) -> Vec<AST> {
        self.args.clone()
    }
    pub fn get_kwargs(&self) -> LinkedHashMap<String, AST> {
        self.kwargs.clone()
    }
    pub fn get_args_format_string(&self) -> String {
        self.args
            .iter()
            .map(|_| "%s".to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn get_presto_query(&self, mut call: String) -> String {
        if self.args.len() > 0 {
            panic!("Do not expect self.args to be > 0 for presto queries.");
        }
        for (k, arg) in &self.kwargs {
            let fmt: String = format!("{{{}}}", k).to_string();
            if let AST::StringLiteral(v) = arg {
                call = call.replace(&fmt, &v.read().unwrap().value());
            }
        }
        call
    }
}
