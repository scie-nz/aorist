use crate::code_block::CodeBlock;
use crate::constraint::{
    AoristStatement, ArgType, LiteralsMap, ParameterTuple, SimpleIdentifier, StringLiteral,
    Subscript,
};
use crate::constraint_state::ConstraintState;
use crate::prefect_singleton::PrefectSingleton;
use inflector::cases::snakecase::to_snake_case;
use linked_hash_set::LinkedHashSet;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct ConstraintBlock<'a> {
    constraint_name: String,
    members: Vec<CodeBlock<'a>>,
    literals: LiteralsMap,
    constraint_states: HashMap<Uuid, Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> ConstraintBlock<'a> {
    pub fn new(
        constraint_name: String,
        members: Vec<CodeBlock<'a>>,
        literals: LiteralsMap,
        constraint_states: HashMap<Uuid, Arc<RwLock<ConstraintState<'a>>>>,
    ) -> Self {
        Self {
            constraint_name,
            members,
            literals,
            constraint_states,
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
                    let uuid = write.get_object_uuids().iter().next().unwrap().clone();
                    let state_rw = self.constraint_states.get(uuid.0).unwrap();
                    let state_read = state_rw.read().unwrap();
                    let task_name: String = state_read.get_task_name();
                    let task_name = task_name.replace(
                        &format!("{}__", to_snake_case(&self.constraint_name)).to_string(),
                        "",
                    );
                    let val = write.value();
                    let param_key = most_frequent_names.get(&val).unwrap();
                    if let Some(key) = param_key {
                        // TODO: need to deal with args
                        let dict_name = ArgType::Subscript(Subscript::new_wrapped(
                            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                format!("params_{}", to_snake_case(&self.constraint_name))
                                    .to_string(),
                            )),
                            ArgType::StringLiteral(Arc::new(RwLock::new(StringLiteral::new(
                                task_name,
                            )))),
                        ));
                        let owner = ArgType::Subscript(Subscript::new_wrapped(
                            dict_name,
                            ArgType::StringLiteral(Arc::new(RwLock::new(StringLiteral::new(
                                key.clone(),
                            )))),
                        ));
                        write.set_owner(owner);
                    }
                }
            }
        }
        drop(read);
    }
    pub fn get_statements(&'a self) -> (Vec<AoristStatement>, LinkedHashSet<String>) {
        for member in &self.members {
            member.register_literals(self.literals.clone());
        }
        self.compute_indirections();
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements(self.literals.clone()))
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
    pub fn get_singletons(&'a self) -> Vec<PrefectSingleton> {
        for member in &self.members {
            member.register_literals(self.literals.clone());
        }
        self.compute_indirections();
        self.members
            .iter()
            .map(|x| {
                x.get_singletons(self.literals.clone())
                    .into_iter()
                    .map(|(_, v)| v)
            })
            .flatten()
            .collect()
    }
}
