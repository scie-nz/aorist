use aorist_primitives::AVec;
mod code_block;
mod constant_r_task;
mod constraint_block;
mod native_r_task;
mod preamble;
mod r_import;
mod task;

pub use constant_r_task::ConstantRTask;
pub use constraint_block::RBasedConstraintBlock;
pub use native_r_task::NativeRTask;
pub use preamble::RPreamble;
pub use r_import::RImport;
pub use task::StandaloneRBasedTask;

use crate::flow::FlowBuilderInput;
use aorist_ast::AST;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;

/// Wrapper type for stuff that gets passed around when building R
/// statements:
/// - A vector of AST objects (main statements),
/// - A set of RPreambles (which have their own imports attached)
/// - A set of imports corresponding to the dialect used.
/// - A comment string
pub struct RFlowBuilderInput {
    statements: AVec<AST>,
    preambles: LinkedHashSet<RPreamble>,
    imports: BTreeSet<RImport>,
    constraint_name: AString,
    constraint_title: Option<AString>,
    constraint_body: Option<AString>,
}
impl FlowBuilderInput for RFlowBuilderInput {
    type ImportType = RImport;
    type PreambleType = RPreamble;

    fn new(
        statements: AVec<AST>,
        preambles: LinkedHashSet<RPreamble>,
        imports: BTreeSet<RImport>,
        constraint_name: AString,
        constraint_title: Option<AString>,
        constraint_body: Option<AString>,
    ) -> Self {
        Self {
            statements,
            preambles,
            imports,
            constraint_name,
            constraint_title,
            constraint_body,
        }
    }
    fn get_statements(&self) -> AVec<AST> {
        self.statements.clone()
    }
    fn get_preambles(&self) -> LinkedHashSet<RPreamble> {
        self.preambles.clone()
    }
    fn get_imports(&self) -> BTreeSet<RImport> {
        self.imports.clone()
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn get_constraint_title(&self) -> Option<AString> {
        self.constraint_title.clone()
    }
    fn get_constraint_body(&self) -> Option<AString> {
        self.constraint_body.clone()
    }
}
