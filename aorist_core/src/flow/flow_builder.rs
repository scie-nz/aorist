use crate::code::Preamble;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder_input::FlowBuilderInput;
use aorist_ast::{Assignment, Dict, SimpleIdentifier, AST};
use aorist_primitives::AoristUniverse;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeMap;
use std::error::Error;
use abi_stable::std_types::RArc;
use std::sync::RwLock;

pub trait FlowBuilderBase<U: AoristUniverse>
where
    Self: Sized,
{
    type T: ETLFlow<U>;
    fn new() -> Self;
}
pub trait FlowBuilderMaterialize<U: AoristUniverse>
where
    Self: Sized,
    Self: FlowBuilderBase<U>,
    Self::BuilderInputType: FlowBuilderInput,
    Self::ErrorType: Error + Send + Sync + 'static,
{
    type BuilderInputType;
    type ErrorType;

    fn materialize(
        &self,
        statements_and_preambles: Vec<Self::BuilderInputType>,
        flow_name: Option<String>,
    ) -> Result<String, Self::ErrorType>;

    fn literals_to_assignments(
        literals: LinkedHashMap<AST, LinkedHashMap<String, Vec<(String, RArc<RwLock<Dict>>)>>>,
    ) -> Vec<AST> {
        let mut assignments: LinkedHashMap<
            String,
            Vec<(AST, Vec<(RArc<RwLock<Dict>>, std::string::String)>)>,
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
        preambles: &LinkedHashSet<<<Self as FlowBuilderBase<U>>::T as ETLFlow<U>>::PreambleType>,
    ) -> Vec<<<Self as FlowBuilderBase<U>>::T as ETLFlow<U>>::ImportType> {
        preambles
            .iter()
            .map(|x| x.get_imports().into_iter())
            .flatten()
            .collect()
    }
}
