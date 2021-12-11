use crate::code::Preamble;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder_input::FlowBuilderInput;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{Assignment, Dict, SimpleIdentifier, AST};
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeMap;
use std::error::Error;

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
        statements_and_preambles: AVec<Self::BuilderInputType>,
        flow_name: Option<AString>,
    ) -> Result<AString, Self::ErrorType>;

    fn literals_to_assignments(
        literals: LinkedHashMap<AST, LinkedHashMap<AString, AVec<(AString, RArc<RRwLock<Dict>>)>>>,
    ) -> AVec<AST> {
        let mut assignments: LinkedHashMap<
            AString,
            AVec<(AST, AVec<(RArc<RRwLock<Dict>>, AString)>)>,
        > = LinkedHashMap::new();
        for (literal, val) in literals.into_iter() {
            for (short_name, keys) in val.into_iter() {
                let mut keys_hist: BTreeMap<AString, usize> = BTreeMap::new();
                let mut rws = AVec::new();
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
                            .to_uppercase()
                            .as_str()
                            .into(),
                        )
                        .or_insert(AVec::new())
                        .push((literal.clone(), rws));
                }
            }
        }
        let mut assignments_ast = AVec::new();
        for (var, vals) in assignments {
            if vals.len() == 1 {
                let lval = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(var));
                let (rval, rws) = vals.into_iter().next().unwrap();
                assignments_ast.push(AST::Assignment(Assignment::new_wrapped(lval.clone(), rval)));
                for (rw, key) in rws {
                    let mut write = rw.write();
                    write.replace_elem(key, lval.clone());
                }
            }
        }
        assignments_ast
    }
    fn get_preamble_imports(
        preambles: &LinkedHashSet<<<Self as FlowBuilderBase<U>>::T as ETLFlow<U>>::PreambleType>,
    ) -> AVec<<<Self as FlowBuilderBase<U>>::T as ETLFlow<U>>::ImportType> {
        preambles
            .iter()
            .map(|x| x.get_imports().into_iter())
            .flatten()
            .collect()
    }
}
