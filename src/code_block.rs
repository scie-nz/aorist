use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::etl_task::{ETLTask, ForLoopETLTask, StandaloneETLTask};
use crate::python::{
    Import, ParameterTuple, Preamble, SimpleIdentifier, StringLiteral, Subscript, AST,
};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub struct CodeBlock<T>
where
    T: ETLSingleton,
{
    tasks_dict: Option<AST>,
    task_identifiers: HashMap<Uuid, AST>,
    etl_tasks: Vec<ETLTask<T>>,
    params: HashMap<String, Option<ParameterTuple>>,
}
impl<'a, T> CodeBlock<T>
where
    T: ETLSingleton,
{
    pub fn get_tasks_dict(&self) -> Option<AST> {
        self.tasks_dict.clone()
    }
    pub fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.task_identifiers.clone()
    }
    pub fn new(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
        tasks_dict: Option<AST>,
        identifiers: &HashMap<Uuid, AST>,
    ) -> Self {
        let with_task_vals = Self::compute_task_vals(members, &constraint_name, &tasks_dict);
        let task_identifiers = with_task_vals
            .iter()
            .map(|(x, rw)| (rw.read().unwrap().get_constraint_uuid(), x.clone()))
            .collect();
        let params = with_task_vals
            .iter()
            .map(|(_, rw)| {
                let x = rw.read().unwrap();
                (x.get_task_name(), x.get_params())
            })
            .collect();

        let tasks = with_task_vals
            .iter()
            .map(|(task_val, rw)| {
                let x = rw.read().unwrap();
                let dep_uuids = x.get_dependencies();
                let dependencies = dep_uuids
                    .iter()
                    .map(|x| identifiers.get(x).unwrap().clone())
                    .collect();
                StandaloneETLTask::new(
                    x.get_task_name(),
                    task_val.clone(),
                    x.get_call(),
                    x.get_params(),
                    dependencies,
                    x.get_preamble(),
                    x.get_dialect(),
                )
            })
            .collect::<Vec<StandaloneETLTask<T>>>();

        let mut compressible: LinkedHashMap<_, Vec<_>> = LinkedHashMap::new();
        let mut etl_tasks: Vec<ETLTask<T>> = Vec::new();

        for task in tasks.into_iter() {
            if task.is_compressible() {
                let key = task.get_compression_key().unwrap();
                compressible.entry(key).or_insert(Vec::new()).push(task);
            } else {
                etl_tasks.push(ETLTask::StandaloneETLTask(task));
            }
        }
        for (compression_key, tasks) in compressible.into_iter() {
            // TODO: this is a magic number
            if tasks.len() > 2 {
                let params_constraint = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    format!("params_{}", constraint_name).to_string(),
                ));
                let compressed_task = ForLoopETLTask::new(
                    params_constraint,
                    compression_key,
                    tasks
                        .into_iter()
                        .map(|x| x.get_uncompressible_part().unwrap())
                        .collect(),
                );
                etl_tasks.push(ETLTask::ForLoopETLTask(compressed_task));
            } else {
                for task in tasks.into_iter() {
                    etl_tasks.push(ETLTask::StandaloneETLTask(task));
                }
            }
        }

        Self {
            tasks_dict,
            etl_tasks,
            task_identifiers,
            params,
        }
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.params.clone()
    }
    /// assigns task values (Python variables in which they will be stored)
    /// to each member of the code block.
    fn compute_task_vals(
        constraints: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: &String,
        tasks_dict: &Option<AST>,
    ) -> Vec<(AST, Arc<RwLock<ConstraintState<'a>>>)> {
        let mut out = Vec::new();
        for rw in constraints.into_iter() {
            let read = rw.read().unwrap();
            let name = read.get_task_name();
            drop(read);
            // TODO: magic number
            let task_val = match tasks_dict {
                None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(name)),
                Some(ref dict) => {
                    let shorter_name =
                        name.replace(&format!("{}__", constraint_name).to_string(), "");

                    AST::Subscript(Subscript::new_wrapped(
                        dict.clone(),
                        AST::StringLiteral(StringLiteral::new_wrapped(shorter_name, false)),
                        false,
                    ))
                }
            };
            out.push((task_val, rw));
        }
        out
    }
    pub fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (Vec<AST>, LinkedHashSet<Preamble>, BTreeSet<Import>) {
        let preambles_and_statements = self
            .etl_tasks
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .map(|x| Preamble::new(x, py))
            .collect::<LinkedHashSet<Preamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        let statements = preambles_and_statements
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .collect::<Vec<_>>();
        (statements, preambles, imports)
    }
}
