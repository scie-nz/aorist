use crate::code::{CodeBlock, CodeBlockWithDefaultConstructor};
use crate::constraint::{OuterConstraint, SatisfiableOuterConstraint};
use crate::endpoints::EndpointConfig;
use crate::flow::{ETLFlow, FlowBuilderInput};
use aorist_ast::AST;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;

pub trait ConstraintBlock<'a, T, C>
where
    T: ETLFlow,
    C: OuterConstraint<'a> + SatisfiableOuterConstraint<'a>,
    Self::C: CodeBlockWithDefaultConstructor<'a, T, C>,
    Self::BuilderInputType: FlowBuilderInput<
        PreambleType = <Self::C as CodeBlock<'a, T, C>>::P,
        ImportType = T::ImportType,
    >,
{
    type C: CodeBlock<'a, T, C>;
    type BuilderInputType;

    fn get_constraint_name(&self) -> String;
    fn get_constraint_title(&self) -> Option<String>;
    fn get_constraint_body(&self) -> Option<String>;
    fn get_code_blocks<'b>(&'a self) -> &'b Vec<Self::C>
    where
        'a: 'b;
    fn get_task_val_assignments(&'a self) -> Vec<AST>;

    fn get_statements(&'a self, endpoints: &EndpointConfig) -> Self::BuilderInputType {
        let preambles_and_statements = self
            .get_code_blocks()
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<_>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<_>>();
        Self::BuilderInputType::new(
            self.get_task_val_assignments()
                .into_iter()
                .chain(preambles_and_statements.into_iter().map(|x| x.0).flatten())
                .collect::<Vec<_>>(),
            preambles,
            imports,
            self.get_constraint_name(),
            self.get_constraint_title(),
            self.get_constraint_body(),
        )
    }

    fn get_identifiers(&self) -> HashMap<Uuid, AST>;
    fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<Self::C>,
        tasks_dict: Option<AST>,
    ) -> Self;
}
