use crate::code_block::CodeBlock;
use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::python::PythonStatementInput;
use crate::python::{Assignment, Dict, Import, ParameterTuple, Preamble, SimpleIdentifier, AST};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

pub struct ConstraintBlock<'a, T>
where
    T: ETLSingleton + 'a,
{
    constraint_name: String,
    title: Option<String>,
    body: Option<String>,
    members: Vec<CodeBlock<'a, T>>,
    singleton_type: PhantomData<T>,
}
impl<'a, T> ConstraintBlock<'a, T>
where
    T: ETLSingleton,
{
    pub fn new(
        constraint_name: String,
        title: Option<String>,
        body: Option<String>,
        members: Vec<CodeBlock<'a, T>>,
    ) -> Self {
        Self {
            constraint_name,
            title,
            body,
            members,
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
    pub fn get_task_val_assignments(&'a self) -> Vec<AST> {
        let mut to_initialize: LinkedHashSet<SimpleIdentifier> = LinkedHashSet::new();
        for block in &self.members {
            for constraint in block.get_constraints() {
                let read = constraint.read().unwrap();
                if let AST::Subscript(sub) = read.get_task_val() {
                    let read2 = sub.read().unwrap();
                    if let AST::SimpleIdentifier(ident) = read2.a() {
                        to_initialize.insert(ident.read().unwrap().clone());
                    }
                }
            }
        }
        to_initialize
            .into_iter()
            .map(|x| {
                AST::Assignment(Assignment::new_wrapped(
                    AST::SimpleIdentifier(Arc::new(RwLock::new(x))),
                    AST::Dict(Dict::new_wrapped(LinkedHashMap::new())),
                ))
            })
            .collect()
    }
    pub fn get_statements(&'a self, endpoints: &EndpointConfig) -> PythonStatementInput {
        let preambles_and_statements = self
            .members
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<Preamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        (
            self.get_task_val_assignments()
                .into_iter()
                .chain(preambles_and_statements.into_iter().map(|x| x.0).flatten())
                .collect::<Vec<_>>(),
            preambles,
            imports,
            self.constraint_name.clone(),
            self.title.clone(),
            self.body.clone(),
        )
    }
    fn get_shorter_task_name(task_name: String) -> String {
        let splits = task_name
            .split("__")
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();
        let mut new_name = task_name.to_string();
        if splits.len() > 2 {
            new_name = format!(
                "{}__{}",
                splits[0].to_string(),
                splits[2..]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()
                    .join("__")
            )
            .to_string();
        } else if splits.len() == 2 {
            new_name = splits[0].to_string();
        } else {
            let splits_inner = splits[0]
                .split("_")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if splits_inner.len() > 2 {
                new_name = format!(
                    "{}_{}",
                    splits_inner[0].to_string(),
                    splits_inner[2..]
                        .iter()
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()
                        .join("_")
                )
                .to_string();
            }
        }
        new_name
    }
    pub fn shorten_task_names(
        members: Vec<CodeBlock<'a, T>>,
        existing_names: &mut HashSet<String>,
    ) -> Vec<CodeBlock<'a, T>> {
        let mut task_names: Vec<(String, Arc<RwLock<ConstraintState<'a>>>)> = Vec::new();
        let mut out = Vec::new();
        for member in members.into_iter() {
            // shorten task names
            let constraints = member.get_constraints();
            for constraint in constraints {
                let fqn = constraint.read().unwrap().get_fully_qualified_task_name();
                task_names.push((fqn, constraint.clone()));
            }
            out.push(member);
        }
        loop {
            let mut should_continue = false;

            let mut proposed_names: Vec<String> = task_names.iter().map(|x| x.0.clone()).collect();
            let mut new_task_names: HashSet<String> = proposed_names.clone().into_iter().collect();
            assert_eq!(proposed_names.len(), new_task_names.len());

            for i in 0..task_names.len() {
                let task_name = proposed_names.get(i).unwrap().clone();
                let new_name = Self::get_shorter_task_name(task_name.clone());

                if new_task_names.contains(&new_name) || existing_names.contains(&new_name) {
                    should_continue = false;
                    break;
                }
                if new_name != task_name {
                    should_continue = true;
                    new_task_names.insert(new_name.clone());
                    proposed_names[i] = new_name;
                }
            }
            if !should_continue {
                break;
            }
            for i in 0..task_names.len() {
                task_names[i].0 = proposed_names[i].clone();
            }
        }
        for (name, rw) in task_names {
            let mut write = rw.write().unwrap();
            existing_names.insert(name.clone());
            write.set_task_name(name.replace("____", "__"));
        }
        out
    }
}
