use crate::code_block::CodeBlock;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::python::PythonStatementInput;
use crate::python::{Assignment, Dict, Import, ParameterTuple, Preamble, SimpleIdentifier, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

pub struct ConstraintBlock<'a, T>
where
    T: ETLSingleton + 'a,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<CodeBlock<'a, T>>,
    singleton_type: PhantomData<T>,
}
impl<'a, T> ConstraintBlock<'a, T>
where
    T: ETLSingleton,
{
    pub fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<CodeBlock<'a, T>>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
            singleton_type: PhantomData,
        }
    }
    pub fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
    pub fn get_task_val_assignments(&'a self) -> Vec<AST> {
        let mut to_initialize: LinkedHashSet<SimpleIdentifier> = LinkedHashSet::new();
        for block in &self.members {
            for constraint in block.get_constraints() {
                let read = constraint.read().unwrap();
                if let AST::Subscript(sub) = read.get_task_val() {
                    let read2 = sub.read().unwrap();
                    if let AST::SimpleIdentifier(ident) = read2.a() {
                        to_initialize.insert(ident.read().unwrap().clone());
                    }
                }
            }
        }
        to_initialize
            .into_iter()
            .map(|x| {
                AST::Assignment(Assignment::new_wrapped(
                    AST::SimpleIdentifier(Arc::new(RwLock::new(x))),
                    AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
                ))
            })
            .collect()
    }
    pub fn get_statements(&'a self, endpoints: &EndpointConfig) -> PythonStatementInput {
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<Preamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        (
            self.get_task_val_assignments()
                .into_iter()
                .chain(preambles_and_statements.into_iter().map(|x| x.0).flatten())
                .collect::<Vec<_>>(),
            preambles,
            imports,
            self.constraint_name.clone(),
        )
    }
}
