use crate::code::CodeBlock;
use crate::constraint::OuterConstraint;
use crate::constraint_block::ConstraintBlock;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use crate::python::PythonBasedCodeBlock;
use crate::python::{Assignment, Dict, PythonFlowBuilderInput, PythonImport, PythonPreamble, AST};
use abi_stable::std_types::ROption;
use aorist_primitives::AOption;
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
    title: AOption<AString>,
    body: AOption<AString>,
    members: AVec<PythonBasedCodeBlock<'a, T, C, U, P>>,
    tasks_dict: AOption<AST>,
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
    fn get_constraint_title(&self) -> AOption<AString> {
        self.title.clone()
    }
    fn get_constraint_body(&self) -> AOption<AString> {
        self.body.clone()
    }
    fn get_code_blocks(&self) -> &AVec<Self::C> {
        &self.members
    }

    fn new(
        constraint_name: AString,
        title: AOption<AString>,
        body: AOption<AString>,
        members: AVec<PythonBasedCodeBlock<'a, T, C, U, P>>,
        tasks_dict: AOption<AST>,
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
            AOption(ROption::RSome(ref val)) => vec![AST::Assignment(Assignment::new_wrapped(
                val.clone(),
                AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
            ))]
            .into_iter()
            .collect(),
            AOption(ROption::RNone) => vec![].into_iter().collect(),
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
    pub fn get_params(&self) -> HashMap<AString, AOption<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
}
