use crate::code::CodeBlock;
use crate::endpoints::EndpointConfig;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::python::PythonBasedCodeBlock;
use crate::python::{Assignment, Dict, PythonImport, PythonPreamble, PythonStatementInput, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use uuid::Uuid;

pub trait ConstraintBlock<'a> {
    type C: CodeBlock;

    fn get_statements(&'a self, endpoints: &EndpointConfig) -> PythonStatementInput;
    fn get_identifiers(&self) -> HashMap<Uuid, AST>;
    fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<Self::C>,
        tasks_dict: Option<AST>,
    ) -> Self;
}

pub struct PythonBasedConstraintBlock<T>
where
    T: ETLFlow,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<PythonBasedCodeBlock<T>>,
    singleton_type: PhantomData<T>,
    tasks_dict: Option<AST>,
}
impl<'a, T> ConstraintBlock<'a> for PythonBasedConstraintBlock<T>
where
    T: ETLFlow,
{
    type C = PythonBasedCodeBlock<T>;

    fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<PythonBasedCodeBlock<T>>,
        tasks_dict: Option<AST>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
            singleton_type: PhantomData,
            tasks_dict,
        }
    }

    fn get_statements(&'a self, endpoints: &EndpointConfig) -> PythonStatementInput {
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<PythonPreamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<PythonImport>>();
        (
            self.get_task_val_assignments()
                .into_iter()
                .chain(preambles_and_statements.into_iter().map(|x| x.0).flatten())
                .collect::<Vec<_>>(),
            preambles,
            imports,
            self.constraint_name.clone(),
            self.title.clone(),
            self.body.clone(),
        )
    }
    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|x| x.get_identifiers().into_iter())
            .flatten()
            .collect()
    }
}

impl<'a, T> PythonBasedConstraintBlock<T>
where
    T: ETLFlow,
{
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
        match &self.tasks_dict {
            Some(ref val) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))],
            None => vec![],
        }
    }
}
