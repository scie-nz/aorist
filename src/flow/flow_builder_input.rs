use crate::code::{Import, Preamble};
use crate::python::AST;
use linked_hash_set::LinkedHashSet;
use std::collections::BTreeSet;

pub trait FlowBuilderInput
where
    Self::ImportType: Import,
    Self::PreambleType: Preamble,
{
    type ImportType;
    type PreambleType;

    fn new(
        statements: Vec<AST>,
        preambles: LinkedHashSet<Self::PreambleType>,
        imports: BTreeSet<Self::ImportType>,
        constraint_name: String,
        constraint_title: Option<String>,
        constraint_body: Option<String>,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_preambles(&self) -> LinkedHashSet<Self::PreambleType>;
    fn get_imports(&self) -> BTreeSet<Self::ImportType>;
    fn get_constraint_name(&self) -> String;
    fn get_constraint_title(&self) -> Option<String>;
    fn get_constraint_body(&self) -> Option<String>;
}
