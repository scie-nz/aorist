use crate::code::{CodeBlock, CodeBlockWithForLoopCompression};
use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleTask, ETLFlow, ETLTask, ForLoopCompressedTask};
use crate::parameter_tuple::ParameterTuple;
use crate::python::{
    ForLoopPythonBasedTask, Formatted, PythonBasedTask, PythonImport, PythonPreamble,
    SimpleIdentifier, StringLiteral, Subscript, AST,
};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use tracing::trace;
use uuid::Uuid;

pub struct PythonBasedCodeBlock<T>
where
    T: ETLFlow,
{
    tasks_dict: Option<AST>,
    task_identifiers: HashMap<Uuid, AST>,
    python_based_tasks: Vec<PythonBasedTask<T>>,
    params: HashMap<String, Option<ParameterTuple>>,
}
impl<T> CodeBlock<T> for PythonBasedCodeBlock<T>
where
    T: ETLFlow,
{
    type P = PythonPreamble;
    type I = PythonImport;
    type E = PythonBasedTask<T>;

    fn construct<'a>(
        tasks_dict: Option<AST>,
        tasks: Vec<Self::E>,
        task_identifiers: HashMap<Uuid, AST>,
        params: HashMap<String, Option<ParameterTuple>>,
    ) -> Self {
        Self {
            tasks_dict,
            python_based_tasks: tasks,
            task_identifiers,
            params,
        }
    }

    fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (
        Vec<AST>,
        LinkedHashSet<PythonPreamble>,
        BTreeSet<PythonImport>,
    ) {
        let preambles_and_statements = self
            .python_based_tasks
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<Vec<_>>();
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .map(|x| PythonPreamble::new(x, py))
            .collect::<LinkedHashSet<PythonPreamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<PythonImport>>();
        let statements = preambles_and_statements
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .collect::<Vec<_>>();
        (statements, preambles, imports)
    }
    fn get_tasks_dict(&self) -> Option<AST> {
        self.tasks_dict.clone()
    }

    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.task_identifiers.clone()
    }

    fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.params.clone()
    }
}
impl<T> CodeBlockWithForLoopCompression<T> for PythonBasedCodeBlock<T>
where
    T: ETLFlow,
{
    fn run_task_compressions(
        compressible: LinkedHashMap<
            <<Self::E as ETLTask<T>>::S as CompressibleTask>::KeyType,
            Vec<<Self::E as ETLTask<T>>::S>,
        >,
        python_based_tasks: &mut Vec<Self::E>,
        constraint_name: String,
    ) {
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
                                    // TODO: pass this to ForLoopETLFlow
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
                            trace!("Compressible kwarg: {}", key);
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

                let compressed_task = ForLoopPythonBasedTask::new(
                    params_constraint,
                    compression_key,
                    maybe_uncompressible,
                    task_id,
                    insert_task_name,
                );
                python_based_tasks.push(PythonBasedTask::ForLoopPythonBasedTask(compressed_task));
            } else {
                for task in tasks.into_iter() {
                    python_based_tasks.push(PythonBasedTask::StandalonePythonBasedTask(task));
                }
            }
        }
    }
}
