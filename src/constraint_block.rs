use crate::code_block::CodeBlock;
use crate::constraint::{ArgType, LiteralsMap, ParameterTuple};
use rustpython_parser::ast::Location;
use std::collections::{HashMap, HashSet};
use inflector::cases::snakecase::to_snake_case;
use uuid::Uuid;

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
    pub fn compute_indirections(&self) {
        let read = self.literals.read().unwrap();
        let all_uuids = read
            .values()
            .map(|x| {
                x.read()
                    .unwrap()
                    .get_object_uuids()
                    .iter()
                    .map(|(x, _)| x.clone())
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .flatten()
            .collect::<HashSet<Uuid>>();
        let most_frequent_names: HashMap<String, Option<String>> = read
            .iter()
            .map(|(k, x)| {
                let read = x.read().unwrap();
                let all_tags = read
                    .get_object_uuids()
                    .iter()
                    .map(|(_, v)| (v.clone().into_iter()))
                    .flatten();
                let mut hist: HashMap<String, usize> = HashMap::new();
                for tag in all_tags {
                    if let Some(t) = tag {
                        *hist.entry(t).or_insert(0) += 1;
                    }
                }
                if hist.len() > 0 {
                    return (
                        k.clone(),
                        Some(hist.into_iter().max_by_key(|(_, v)| *v).unwrap().0),
                    );
                }
                (k.clone(), None)
            })
            .collect();

        println!("Constraint: {}", &self.constraint_name);
        for (k, v) in read.iter() {
            let mut write = v.write().unwrap();
            println!("{} => {}", k, write.get_object_uuids().len());
            if all_uuids.len() > 1 && write.get_object_uuids().len() > all_uuids.len() / 2 {
                let val = write.value();
                let possible_name = most_frequent_names.get(&val).unwrap();
                if let Some(ref name) = possible_name {
                    let proposed_name =
                        format!("{}_{}", to_snake_case(&self.constraint_name), name).to_string();
                    if proposed_name.len() < name.len() || write.is_multiline() {
                        let referenced_by = ArgType::SimpleIdentifier(proposed_name);
                        write.set_referenced_by(Box::new(referenced_by));
                        println!("Set indirection for {}", k);
                    }
                }
            }
        }
        drop(read);
    }
    pub fn render(&self, location: Location) {
        self.compute_indirections();
        // TODO: rename print_call
        for (_i, member) in self.members.iter().enumerate() {
            member.print_call(location);
        }
    }
}
