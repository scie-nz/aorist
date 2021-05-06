use crate::code::CodeBlock;
use crate::constraint_block::ConstraintBlock;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::python::PythonBasedCodeBlock;
use crate::python::{Assignment, Dict, PythonFlowBuilderInput, PythonImport, PythonPreamble, AST};
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PythonBasedConstraintBlock<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<PythonBasedCodeBlock<T>>,
    singleton_type: PhantomData<T>,
    tasks_dict: Option<AST>,
}
impl<'a, T> ConstraintBlock<'a, T> for PythonBasedConstraintBlock<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    type C = PythonBasedCodeBlock<T>;
    type BuilderInputType = PythonFlowBuilderInput;

    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn get_constraint_title(&self) -> Option<String> {
        self.title.clone()
    }
    fn get_constraint_body(&self) -> Option<String> {
        self.body.clone()
    }
    fn get_code_blocks<'b>(&'a self) -> &'b Vec<Self::C>
    where
        'a: 'b,
    {
        &self.members
    }

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
    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|x| x.get_identifiers().into_iter())
            .flatten()
            .collect()
    }

    fn get_task_val_assignments(&'a self) -> Vec<AST> {
        match &self.tasks_dict {
            Some(ref val) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))],
            None => vec![],
        }
    }
}

impl<'a, T> PythonBasedConstraintBlock<T>
where
    T: ETLFlow<ImportType = PythonImport, PreambleType = PythonPreamble>,
{
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
}
