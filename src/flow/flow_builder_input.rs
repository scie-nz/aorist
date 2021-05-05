use crate::python::{AST, PythonPreamble, PythonImport};
use std::collections::BTreeSet;
use linked_hash_set::LinkedHashSet;

pub trait FlowBuilderInput {
    fn new(
        statements: Vec<AST>,
        preambles: LinkedHashSet<PythonPreamble>,
        imports: BTreeSet<PythonImport>,
        constraint_name: String,
        constraint_title: Option<String>,
        constraint_body: Option<String>,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_preambles(&self) -> LinkedHashSet<PythonPreamble>;
    fn get_imports(&self) -> BTreeSet<PythonImport>;
    fn get_constraint_name(&self) -> String;
    fn get_constraint_title(&self) -> Option<String>;
    fn get_constraint_body(&self) -> Option<String>;
}

