use crate::code_block::CodeBlock;
use indoc::formatdoc;
use std::collections::{HashMap, HashSet};
pub struct ConstraintBlock<'a> {
    constraint_name: String,
    members: Vec<CodeBlock<'a>>,
}
impl<'a> ConstraintBlock<'a> {
    pub fn new(constraint_name: String, members: Vec<CodeBlock<'a>>) -> Self {
        Self {
            constraint_name,
            members,
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
    pub fn get_params(&self) -> HashMap<String, Option<String>> {
        self.members
            .iter()
            .map(|x| x.get_params().into_iter())
            .flatten()
            .collect()
    }
    pub fn print_params(&self) {
        // TODO: needs refactoring
        // - param keys are wrong
        // - unnecessary for singletons
        let params = self
            .get_params()
            .into_iter()
            .filter(|(_, v)| v.is_some())
            .collect::<Vec<_>>();
        if params.len() > 0 {
            println!(
                "{}",
                formatdoc!(
                    "
            params_{constraint} = {{
                {params}
            }}
                ",
                    constraint = self.get_constraint_name(),
                    params = params
                        .into_iter()
                        .map(|(k, v)| format!(
                            "'{constraint}_{k}': ({v})",
                            constraint = self.get_constraint_name(),
                            k = k,
                            v = v.unwrap()
                        )
                        .to_string())
                        .collect::<Vec<String>>()
                        .join(",\n    "),
                )
            );
        }
        for (_i, member) in self.members.iter().enumerate() {
            member.print_call(self.get_constraint_name());
        }
    }
}
