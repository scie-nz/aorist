
use crate::code::CodeBlock;
use crate::constraint::OuterConstraint;
use crate::constraint_block::ConstraintBlock;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use crate::python::PythonBasedCodeBlock;
use crate::python::{Assignment, Dict, PythonFlowBuilderInput, PythonImport, PythonPreamble, AST};
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct PythonBasedConstraintBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    constraint_name: AString,
    title: Option<AString>,
    body: Option<AString>,
    members: AVec<PythonBasedCodeBlock<'a, T, C, U, P>>,
    tasks_dict: Option<AST>,
    _lt: PhantomData<&'a ()>,
    _constraint: PhantomData<C>,
}
impl<'a, T, C, U, P> ConstraintBlock<'a, T, C, U, P> for PythonBasedConstraintBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    type C = PythonBasedCodeBlock<'a, T, C, U, P>;
    type BuilderInputType = PythonFlowBuilderInput;

    fn get_constraint_name(&self) -> AString {
        self.constraint_name.clone()
    }
    fn get_constraint_title(&self) -> Option<AString> {
        self.title.clone()
    }
    fn get_constraint_body(&self) -> Option<AString> {
        self.body.clone()
    }
    fn get_code_blocks(&self) -> &AVec<Self::C> {
        &self.members
    }

    fn new(
        constraint_name: AString,
        title: Option<AString>,
        body: Option<AString>,
        members: AVec<PythonBasedCodeBlock<'a, T, C, U, P>>,
        tasks_dict: Option<AST>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
            tasks_dict,
            _lt: PhantomData,
            _constraint: PhantomData,
        }
    }
    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.members
            .iter()
            .map(|x| x.get_identifiers().into_iter())
            .flatten()
            .collect()
    }

    fn get_task_val_assignments(&self) -> AVec<AST> {
        match &self.tasks_dict {
            Some(ref val) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))],
            None => vec![],
        }
    }
}

impl<'a, T, C, U, P> PythonBasedConstraintBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    pub fn get_params(&self) -> HashMap<AString, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
}
