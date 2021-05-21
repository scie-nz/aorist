use crate::code::Preamble;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder_input::FlowBuilderInput;
use aorist_ast::{Assignment, Dict, SimpleIdentifier, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

pub trait FlowBuilderBase
where
    Self: Sized,
{
    type T: ETLFlow;
    fn new() -> Self;
}
pub trait FlowBuilderMaterialize
where
    Self: Sized,
    Self: FlowBuilderBase,
    Self::BuilderInputType: FlowBuilderInput,
    Self::ErrorType: Error + Send + Sync + 'static,
{
    type BuilderInputType;
    type ErrorType;

    fn materialize(
        &self,
        statements_and_preambles: Vec<Self::BuilderInputType>,
    ) -> Result<String, Self::ErrorType>;

    fn extract_literals(
        ast: &AST,
        short_name: &String,
        literals: &mut LinkedHashMap<AST, LinkedHashMap<String, Vec<(String, Arc<RwLock<Dict>>)>>>,
    ) {
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
    fn literals_to_assignments(
        literals: LinkedHashMap<AST, LinkedHashMap<String, Vec<(String, Arc<RwLock<Dict>>)>>>,
    ) -> Vec<AST> {
        let mut assignments: LinkedHashMap<
            String,
            Vec<(AST, Vec<(Arc<RwLock<Dict>>, std::string::String)>)>,
        > = LinkedHashMap::new();
        for (literal, val) in literals.into_iter() {
            for (short_name, keys) in val.into_iter() {
                let mut keys_hist: BTreeMap<String, usize> = BTreeMap::new();
                let mut rws = Vec::new();
                for (key, rw) in keys {
                    *keys_hist.entry(key.clone()).or_insert(1) += 1;
                    rws.push((rw, key));
                }
                if keys_hist.len() == 1 {
                    assignments
                        .entry(
                            format!(
                                "{}__{}",
                                keys_hist.into_iter().next().unwrap().0,
                                short_name
                            )
                            .to_string()
                            .to_uppercase(),
                        )
                        .or_insert(Vec::new())
                        .push((literal.clone(), rws));
                }
            }
        }
        let mut assignments_ast = Vec::new();
        for (var, vals) in assignments {
            if vals.len() == 1 {
                let lval = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(var));
                let (rval, rws) = vals.into_iter().next().unwrap();
                assignments_ast.push(AST::Assignment(Assignment::new_wrapped(lval.clone(), rval)));
                for (rw, key) in rws {
                    let mut write = rw.write().unwrap();
                    write.replace_elem(key, lval.clone());
                }
            }
        }
        assignments_ast
    }
    fn get_preamble_imports(
        preambles: &LinkedHashSet<<<Self as FlowBuilderBase>::T as ETLFlow>::PreambleType>,
    ) -> Vec<<<Self as FlowBuilderBase>::T as ETLFlow>::ImportType> {
        preambles
            .iter()
            .map(|x| x.get_imports().into_iter())
            .flatten()
            .collect()
    }
}
