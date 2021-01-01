use crate::code_block::CodeBlock;
use crate::constraint::{LiteralsMap, ParameterTuple};
use rustpython_parser::ast::Location;
use std::collections::{HashMap, HashSet};

pub struct ConstraintBlock<'a> {
    constraint_name: String,
    members: Vec<CodeBlock<'a>>,
    literals: LiteralsMap,
}
impl<'a> ConstraintBlock<'a> {
    pub fn new(
        constraint_name: String,
        members: Vec<CodeBlock<'a>>,
        literals: LiteralsMap,
    ) -> Self {
        Self {
            constraint_name,
            members,
            literals,
        }
    }
    pub fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    pub fn get_preambles(&self) -> HashSet<String> {
        self.members
            .iter()
            .map(|x| x.get_preambles().into_iter())
            .flatten()
            .collect()
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
    pub fn render(&self, location: Location) {
        // TODO: rename print_call
        for (_i, member) in self.members.iter().enumerate() {
            member.print_call(location);
        }
    }
}
