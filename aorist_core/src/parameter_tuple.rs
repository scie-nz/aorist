
use aorist_ast::{AncestorRecord, List, StringLiteral, AST};
use aorist_primitives::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ParameterTuple {
    pub args: AVec<AST>,
    pub kwargs: LinkedHashMap<AString, AST>,
}
pub type ParameterTupleDedupKey = (usize, AVec<AString>);
impl ParameterTuple {
    pub fn set_ancestors(&self, ancestors: AVec<AncestorRecord>) {
        for arg in self.args.iter() {
            arg.set_ancestors(ancestors.clone());
        }
        for v in self.kwargs.values() {
            v.set_ancestors(ancestors.clone());
        }
    }
    pub fn populate_python_dict(&self, dict: &mut LinkedHashMap<AString, AST>) {
        if self.args.len() > 0 {
            dict.insert(
                "args".into(),
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
        args_v: AVec<AString>,
        kwargs_v: LinkedHashMap<AString, AString>,
        is_sql: bool,
    ) -> Self {
        let args = args_v
            .into_iter()
            .map(|x| AST::StringLiteral(StringLiteral::new_wrapped(x, is_sql)))
            .collect::<AVec<_>>();
        let kwargs = kwargs_v
            .into_iter()
            .map(|(k, v)| (k, AST::StringLiteral(StringLiteral::new_wrapped(v, is_sql))))
            .collect::<LinkedHashMap<_, _>>();
        Self { args, kwargs }
    }
    pub fn get_args(&self) -> AVec<AST> {
        self.args.clone()
    }
    pub fn get_kwargs(&self) -> LinkedHashMap<AString, AST> {
        self.kwargs.clone()
    }
    pub fn get_args_format_string(&self) -> AString {
        self.args
            .iter()
            .map(|_| "%s".into())
            .collect::<AVec<String>>()
            .join(" ")
            .as_str()
            .into()
    }
    pub fn get_presto_query(&self, mut call: AString) -> AString {
        if self.args.len() > 0 {
            panic!("Do not expect self.args to be > 0 for presto queries.");
        }
        for (k, arg) in &self.kwargs {
            let fmt = format!("{{{}}}", k).as_str().to_string();
            if let AST::StringLiteral(v) = arg {
                call = call
                    .as_str()
                    .to_string()
                    .replace(&fmt, v.read().value().as_str())
                    .as_str()
                    .into();
            }
        }
        call
    }
}
