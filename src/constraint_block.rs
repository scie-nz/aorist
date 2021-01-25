use crate::code_block::CodeBlock;
use crate::constraint::{AoristStatement, ArgType, LiteralsMap, ParameterTuple, SimpleIdentifier};
use inflector::cases::snakecase::to_snake_case;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use std::marker::PhantomData;
use crate::etl_singleton::ETLSingleton;

pub struct ConstraintBlock<'a, T>
where T: ETLSingleton {
    constraint_name: String,
    members: Vec<CodeBlock<'a, T>>,
    literals: LiteralsMap,
    singleton_type: PhantomData<T>,
}
impl<'a, T> ConstraintBlock<'a, T>
where T: ETLSingleton {
    pub fn new(
        constraint_name: String,
        members: Vec<CodeBlock<'a, T>>,
        literals: LiteralsMap,
    ) -> Self {
        Self {
            constraint_name,
            members,
            literals,
            singleton_type: PhantomData,
        }
    }
    pub fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
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

        for (_k, v) in read.iter() {
            let mut write = v.write().unwrap();
            if all_uuids.len() > 1 {
                let num_objects_covered = write.get_object_uuids().len();
                let num_objects_total = all_uuids.len();
                if num_objects_covered > num_objects_total / 2 {
                    let val = write.value();
                    let possible_name = most_frequent_names.get(&val).unwrap();
                    if let Some(ref name) = possible_name {
                        let proposed_name =
                            format!("{}_{}", to_snake_case(&self.constraint_name), name)
                                .to_string();
                        if proposed_name.len() < name.len() || write.is_multiline() {
                            let owner = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                proposed_name,
                            ));
                            write.set_owner(owner);
                        }
                    }
                } else if num_objects_covered == 1 {
                }
            }
        }
        drop(read);
    }
    pub fn get_statements(&'a self) -> (Vec<AoristStatement>, LinkedHashSet<String>) {
        self.compute_indirections();
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements())
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<String>>();
        (
            preambles_and_statements
                .into_iter()
                .map(|x| x.0)
                .flatten()
                .collect::<Vec<_>>(),
            preambles,
        )
    }
}
