use crate::constraint_state::ConstraintState;
use crate::endpoints::EndpointConfig;
use crate::etl_singleton::ETLSingleton;
use crate::etl_task::{ETLTask, ForLoopETLTask, StandaloneETLTask};
use crate::python::{
    Formatted, Import, ParameterTuple, Preamble, SimpleIdentifier, StringLiteral, Subscript, AST,
};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
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
        for (mut compression_key, tasks) in compressible.into_iter() {
            let num_tasks = tasks.len();
            // TODO: this is a magic number
            if num_tasks > 1 {
                let params_constraint = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    format!("params_{}", constraint_name).to_string(),
                ));
                let mut maybe_uncompressible = tasks
                    .into_iter()
                    .map(|x| x.get_uncompressible_part().unwrap())
                    .collect::<Vec<_>>();

                let mut deps: HashMap<AST, HashSet<String>> = HashMap::new();
                let mut kwargs: LinkedHashMap<String, HashMap<AST, HashSet<String>>> =
                    LinkedHashMap::new();
                let mut kwargs_by_task_id: LinkedHashMap<(String, AST), HashSet<String>> =
                    LinkedHashMap::new();
                let mut full_task_ids: LinkedHashMap<AST, HashSet<String>> = LinkedHashMap::new();

                for t in &maybe_uncompressible {
                    for dep in &t.deps {
                        deps.entry(dep.clone())
                            .or_insert(HashSet::new())
                            .insert(t.task_id.clone());
                    }
                    let task_id_subscript = t.task_id.split("__").last().unwrap().to_string();
                    let replaced = t.task_id.replace(&task_id_subscript, "{t}");
                    let ident =
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".to_string()));
                    let mut kw = LinkedHashMap::new();
                    kw.insert("t".to_string(), ident);
                    let replacement = AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(StringLiteral::new_wrapped(replaced, false)),
                        kw,
                    ));
                    full_task_ids
                        .entry(replacement)
                        .or_insert(HashSet::new())
                        .insert(t.task_id.clone());

                    if let Some(ref p) = t.params {
                        for (key, val) in p.kwargs.iter() {
                            let val_no_ancestors = val.clone_without_ancestors();
                            if let AST::StringLiteral(rw) = val {
                                let x = rw.read().unwrap();
                                if x.value() == task_id_subscript {
                                    // TODO: pass this to ForLoopETLSingleton
                                    let ident = AST::SimpleIdentifier(
                                        SimpleIdentifier::new_wrapped("t".to_string()),
                                    );
                                    kwargs_by_task_id
                                        .entry((key.clone(), ident))
                                        .or_insert(HashSet::new())
                                        .insert(t.task_id.clone());
                                } else {
                                    let replaced = x.value().replace(&task_id_subscript, "{t}");
                                    if replaced != x.value() {
                                        let ident = AST::SimpleIdentifier(
                                            SimpleIdentifier::new_wrapped("t".to_string()),
                                        );
                                        let mut kw = LinkedHashMap::new();
                                        kw.insert("t".to_string(), ident);
                                        let replacement = AST::Formatted(Formatted::new_wrapped(
                                            AST::StringLiteral(StringLiteral::new_wrapped(
                                                replaced, false,
                                            )),
                                            kw,
                                        ));
                                        kwargs_by_task_id
                                            .entry((key.clone(), replacement))
                                            .or_insert(HashSet::new())
                                            .insert(t.task_id.clone());
                                    }
                                }
                            }

                            kwargs
                                .entry(key.clone())
                                .or_insert(HashMap::new())
                                .entry(val_no_ancestors)
                                .or_insert(HashSet::new())
                                .insert(t.task_id.clone());
                        }
                    }
                }
                let compressible_deps = deps
                    .into_iter()
                    .filter(|(_k, v)| v.len() == num_tasks)
                    .map(|(k, _)| k)
                    .collect::<HashSet<AST>>();
                let compressible_kwargs_by_task_id = kwargs_by_task_id
                    .into_iter()
                    .filter(|(_k, v)| v.len() == num_tasks)
                    .map(|(k, _)| k)
                    .collect::<LinkedHashSet<_>>();
                let mut compressible_kwargs = kwargs
                    .into_iter()
                    .map(|(key, val)| {
                        (
                            key,
                            val.into_iter()
                                .filter(|(_k, v)| v.len() == num_tasks)
                                .map(|(k, _)| k)
                                .next(),
                        )
                    })
                    .filter(|(_k, v)| v.is_some())
                    .map(|(k, v)| (k, v.unwrap()))
                    .collect::<LinkedHashMap<String, AST>>();

                for (key, val) in compressible_kwargs_by_task_id.iter() {
                    compressible_kwargs.insert(key.clone(), val.clone());
                }

                for t in maybe_uncompressible.iter_mut() {
                    let mut new_deps = Vec::new();
                    for dep in t.deps.iter() {
                        if !compressible_deps.contains(dep) {
                            new_deps.push(dep.clone());
                        }
                    }
                    if let Some(ref mut p) = t.params {
                        for key in compressible_kwargs.keys() {
                            //println!("Compressible kwarg: {}", key);
                            p.kwargs.remove(key);
                        }
                        for (key, _) in compressible_kwargs_by_task_id.iter() {
                            p.kwargs.remove(key);
                        }
                    }
                    t.deps = new_deps;
                }
                compression_key.deps = compressible_deps.into_iter().collect();
                compression_key.kwargs = compressible_kwargs;

                // TODO: insert_task_name should not be necessary
                let (task_id, insert_task_name) = match full_task_ids.len() {
                    1 => (full_task_ids.into_iter().next().unwrap().0, false),
                    _ => (
                        AST::Subscript(Subscript::new_wrapped(
                            params_constraint.clone(),
                            AST::StringLiteral(StringLiteral::new_wrapped(
                                "task_id".to_string(),
                                false,
                            )),
                            false,
                        )),
                        true,
                    ),
                };

                let compressed_task = ForLoopETLTask::new(
                    params_constraint,
                    compression_key,
                    maybe_uncompressible,
                    task_id,
                    insert_task_name,
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
