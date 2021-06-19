use crate::code::CodeBlock;
use crate::constraint::{OuterConstraint, SatisfiableOuterConstraint};
use crate::constraint_block::ConstraintBlock;
use crate::flow::ETLFlow;
use crate::parameter_tuple::ParameterTuple;
use crate::r::code_block::RBasedCodeBlock;
use crate::r::r_import::RImport;
use crate::r::RFlowBuilderInput;
use crate::r::RPreamble;
use aorist_ast::{Assignment, Dict, AST};
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

pub struct RBasedConstraintBlock<'a, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a> + SatisfiableOuterConstraint<'a>,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<RBasedCodeBlock<'a, T, C>>,
    tasks_dict: Option<AST>,
    _lt: PhantomData<&'a ()>,
    _constraint: PhantomData<C>,
}
impl<'a, T, C> ConstraintBlock<'a, T, C> for RBasedConstraintBlock<'a, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a> + SatisfiableOuterConstraint<'a>,
{
    type C = RBasedCodeBlock<'a, T, C>;
    type BuilderInputType = RFlowBuilderInput;

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
        members: Vec<RBasedCodeBlock<'a, T, C>>,
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

impl<'a, T, C> RBasedConstraintBlock<'a, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a> + SatisfiableOuterConstraint<'a>,
{
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
}
