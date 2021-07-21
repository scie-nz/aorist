use crate::code::{Import, Preamble};
use aorist_ast::{AST, Dict};
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;
use linked_hash_map::LinkedHashMap;
use std::sync::{Arc, RwLock};

pub trait FlowBuilderInput
where
    Self::ImportType: Import,
    Self::PreambleType: Preamble,
{
    type ImportType;
    type PreambleType;

    fn new(
        statements: Vec<AST>,
        preambles: LinkedHashSet<Self::PreambleType>,
        imports: BTreeSet<Self::ImportType>,
        constraint_name: String,
        constraint_title: Option<String>,
        constraint_body: Option<String>,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_preambles(&self) -> LinkedHashSet<Self::PreambleType>;
    fn get_imports(&self) -> BTreeSet<Self::ImportType>;
    fn get_constraint_name(&self) -> String;
    fn get_constraint_title(&self) -> Option<String>;
    fn get_constraint_body(&self) -> Option<String>;
    fn get_block_comment(&self) -> String {
        match self.get_constraint_title() {
            Some(t) => match self.get_constraint_body() {
                Some(b) => format!(
                    "## {}\n{}",
                    t,
                    b.split("\n")
                        .map(|x| format!("# {}", x).to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
                .to_string(),
                None => format!("## {}", t).to_string(),
            },
            None => format!("## {}", self.get_constraint_name()).to_string(),
        }
    }
    
    fn extract_literals(
        &self,
        literals: &mut LinkedHashMap<AST, LinkedHashMap<String, Vec<(String, Arc<RwLock<Dict>>)>>>,
    ) {
        let short_name = &self.get_constraint_name();
        for ast in &self.get_statements() {
            if let AST::Assignment(rw) = ast {
                let assign = rw.read().unwrap();
                if let AST::Dict(dict_rw) = assign.call() {
                    let dict = dict_rw.read().unwrap();
                    for (_task_key, task_val) in dict.elems() {
                        if let AST::Dict(param_dict_rw) = task_val {
                            let param_dict = param_dict_rw.read().unwrap();
                            for (key, val) in param_dict.elems() {
                                if let Some(_ancestors) = val.get_ancestors() {
                                    let is_long_literal = match val {
                                        AST::StringLiteral(ref x) => {
                                            x.read().unwrap().value().len()
                                                > short_name.len() + key.len() + 2
                                        }
                                        _ => true,
                                    };
                                    if is_long_literal {
                                        literals
                                            .entry(val.clone_without_ancestors())
                                            .or_insert(LinkedHashMap::new())
                                            .entry(short_name.to_string())
                                            .or_insert(Vec::new())
                                            .push((key.to_string(), param_dict_rw.clone()));
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
