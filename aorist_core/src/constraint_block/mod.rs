use crate::code::{CodeBlock, CodeBlockWithDefaultConstructor};
use crate::constraint::OuterConstraint;
use crate::flow::{ETLFlow, FlowBuilderInput};
use crate::program::TOuterProgram;
use aorist_ast::AST;
use aorist_primitives::AoristUniverse;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec};
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};

pub trait ConstraintBlock<'a, T, C, U, P>
where
    T: ETLFlow<U>,
    U: AoristUniverse,
    C: OuterConstraint<'a>,
    P: TOuterProgram<TAncestry = C::TAncestry>,
    Self::C: CodeBlockWithDefaultConstructor<'a, T, C, U, P>,
    Self::BuilderInputType: FlowBuilderInput<
        PreambleType = <Self::C as CodeBlock<'a, T, C, U, P>>::P,
        ImportType = T::ImportType,
    >,
{
    type C: CodeBlock<'a, T, C, U, P>;
    type BuilderInputType;

    fn get_constraint_name(&self) -> AString;
    fn get_constraint_title(&self) -> AOption<AString>;
    fn get_constraint_body(&self) -> AOption<AString>;
    fn get_code_blocks(&self) -> &AVec<Self::C>;
    fn get_task_val_assignments(&self) -> AVec<AST>;

    fn get_statements(&self, endpoints: U::TEndpoints) -> Self::BuilderInputType {
        let preambles_and_statements = self
            .get_code_blocks()
            .iter()
            .map(|x| x.get_statements(endpoints.clone()))
            .collect::<AVec<_>>();
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
                .collect::<AVec<_>>(),
            preambles,
            imports,
            self.get_constraint_name(),
            self.get_constraint_title(),
            self.get_constraint_body(),
        )
    }

    fn get_identifiers(&self) -> HashMap<AUuid, AST>;
    fn new(
        constraint_name: AString,
        title: AOption<AString>,
        body: AOption<AString>,
        members: AVec<Self::C>,
        tasks_dict: AOption<AST>,
    ) -> Self;
}
