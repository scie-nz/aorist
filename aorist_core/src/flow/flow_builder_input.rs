use crate::code::{Import, Preamble};
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{Dict, AST};
use aorist_primitives::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;

pub trait FlowBuilderInput
where
    Self::ImportType: Import,
    Self::PreambleType: Preamble,
{
    type ImportType;
    type PreambleType;

    fn new(
        statements: AVec<AST>,
        preambles: LinkedHashSet<Self::PreambleType>,
        imports: BTreeSet<Self::ImportType>,
        constraint_name: AString,
        constraint_title: Option<AString>,
        constraint_body: Option<AString>,
    ) -> Self;
    fn get_statements(&self) -> AVec<AST>;
    fn get_preambles(&self) -> LinkedHashSet<Self::PreambleType>;
    fn get_imports(&self) -> BTreeSet<Self::ImportType>;
    fn get_constraint_name(&self) -> AString;
    fn get_constraint_title(&self) -> Option<AString>;
    fn get_constraint_body(&self) -> Option<AString>;
    fn get_block_comment(&self) -> AString {
        match self.get_constraint_title() {
            Some(t) => match self.get_constraint_body() {
                Some(b) => format!(
                    "## {}\n{}",
                    t,
                    b.as_str()
                        .to_string()
                        .split("\n")
                        .map(|x| format!("# {}", x).to_string())
                        .collect::<AVec<String>>()
                        .join("\n")
                        .as_str()
                )
                .as_str()
                .into(),
                None => format!("## {}", t).as_str().into(),
            },
            None => format!("## {}", self.get_constraint_name()).as_str().into(),
        }
    }

    fn extract_literals(
        &self,
        literals: &mut LinkedHashMap<
            AST,
            LinkedHashMap<AString, AVec<(AString, RArc<RRwLock<Dict>>)>>,
        >,
    ) {
        let short_name = &self.get_constraint_name();
        for ast in self.get_statements().iter() {
            if let AST::Assignment(rw) = ast {
                let assign = rw.read();
                if let AST::Dict(dict_rw) = assign.call() {
                    let dict = dict_rw.read();
                    for (_task_key, task_val) in dict.elems() {
                        if let AST::Dict(param_dict_rw) = task_val {
                            let param_dict = param_dict_rw.read();
                            for (key, val) in param_dict.elems() {
                                if let Some(_ancestors) = val.get_ancestors() {
                                    let is_long_literal = match val {
                                        AST::StringLiteral(ref x) => {
                                            x.read().value().len()
                                                > short_name.len() + key.len() + 2
                                        }
                                        _ => true,
                                    };
                                    if is_long_literal {
                                        literals
                                            .entry(val.clone_without_ancestors())
                                            .or_insert(LinkedHashMap::new())
                                            .entry(short_name.clone())
                                            .or_insert(AVec::new())
                                            .push((key.into(), param_dict_rw.clone()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
