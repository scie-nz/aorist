use crate::code::{CodeBlock, CodeBlockWithForLoopCompression};
use crate::constraint::OuterConstraint;
use crate::flow::{CompressibleTask, ETLFlow, ETLTask, ForLoopCompressedTask};
use crate::parameter_tuple::ParameterTuple;
use crate::program::TOuterProgram;
use crate::python::{
    ForLoopPythonBasedTask, Formatted, PythonBasedTask, PythonImport, PythonPreamble,
    SimpleIdentifier, StringLiteral, Subscript, AST,
};
use abi_stable::std_types::ROption;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;
use tracing::trace;

pub struct PythonBasedCodeBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    tasks_dict: AOption<AST>,
    task_identifiers: HashMap<AUuid, AST>,
    python_based_tasks: AVec<PythonBasedTask<T, U>>,
    params: HashMap<AString, AOption<ParameterTuple>>,
    _lt: PhantomData<&'a ()>,
    _constraint: PhantomData<C>,
    _program: PhantomData<P>,
}
impl<'a, T, C, U, P> CodeBlock<'a, T, C, U, P> for PythonBasedCodeBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    type P = PythonPreamble;
    type E = PythonBasedTask<T, U>;

    fn construct(
        tasks_dict: AOption<AST>,
        tasks: AVec<Self::E>,
        task_identifiers: HashMap<AUuid, AST>,
        params: HashMap<AString, AOption<ParameterTuple>>,
    ) -> Self {
        Self {
            tasks_dict,
            python_based_tasks: tasks,
            task_identifiers,
            params,
            _lt: PhantomData,
            _constraint: PhantomData,
            _program: PhantomData,
        }
    }

    fn get_statements(
        &self,
        endpoints: U::TEndpoints,
    ) -> (
        AVec<AST>,
        LinkedHashSet<PythonPreamble>,
        BTreeSet<PythonImport>,
    ) {
        let preambles_and_statements = self
            .python_based_tasks
            .iter()
            .map(|x| x.get_statements(endpoints.clone()))
            .collect::<AVec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
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
            .collect::<AVec<_>>();
        (statements, preambles, imports)
    }
    fn get_tasks_dict(&self) -> AOption<AST> {
        self.tasks_dict.clone()
    }

    fn get_identifiers(&self) -> HashMap<AUuid, AST> {
        self.task_identifiers.clone()
    }

    fn get_params(&self) -> HashMap<AString, AOption<ParameterTuple>> {
        self.params.clone()
    }
}
impl<'a, T, C, U, P> CodeBlockWithForLoopCompression<'a, T, C, U, P>
    for PythonBasedCodeBlock<'a, T, C, U, P>
where
    T: ETLFlow<U, ImportType = PythonImport, PreambleType = PythonPreamble>,
    C: OuterConstraint<'a>,
    U: AoristUniverse,
    P: TOuterProgram<TAncestry = C::TAncestry>,
{
    fn run_task_compressions(
        compressible: LinkedHashMap<
            <<Self::E as ETLTask<T, U>>::S as CompressibleTask>::KeyType,
            AVec<<Self::E as ETLTask<T, U>>::S>,
        >,
        python_based_tasks: &mut AVec<Self::E>,
        constraint_name: AString,
        render_dependencies: bool,
    ) {
        for (mut compression_key, tasks) in compressible.into_iter() {
            let num_tasks = tasks.len();
            // TODO: this is a magic number
            if num_tasks > 1 {
                trace!(
                    "Running compression for {} tasks for constraint {}",
                    num_tasks,
                    constraint_name
                );
                let params_constraint = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    format!("params_{}", constraint_name).as_str().into(),
                ));
                let mut maybe_uncompressible = tasks
                    .into_iter()
                    .map(|x| x.get_uncompressible_part().unwrap())
                    .collect::<AVec<_>>();
                trace!(
                    "There are {} maybe_uncompressible tasks",
                    maybe_uncompressible.len()
                );
                for v in maybe_uncompressible.iter() {
                    trace!("-- {:?} : {:?}", v.dict, v.params);
                }
                let distinct_keys = maybe_uncompressible
                    .iter()
                    .map(|x| x.dict.clone())
                    .collect::<std::collections::HashSet<_>>();
                if distinct_keys.len() < maybe_uncompressible.len() {
                    panic!("Tasks with same keys in for loop compression.");
                }

                let mut deps: HashMap<AST, HashSet<AString>> = HashMap::new();
                let mut kwargs: LinkedHashMap<AString, HashMap<AST, HashSet<AString>>> =
                    LinkedHashMap::new();
                let mut kwargs_by_task_id: LinkedHashMap<(AString, AST), HashSet<AString>> =
                    LinkedHashMap::new();
                let mut full_task_ids: LinkedHashMap<AST, HashSet<AString>> = LinkedHashMap::new();

                for t in maybe_uncompressible.iter() {
                    for dep in t.deps.iter() {
                        deps.entry(dep.clone())
                            .or_insert(HashSet::new())
                            .insert(t.task_id.clone());
                    }
                    let task_id_subscript = t
                        .task_id
                        .as_str()
                        .to_string()
                        .split("__")
                        .last()
                        .unwrap()
                        .to_string();
                    let replaced = t
                        .task_id
                        .as_str()
                        .to_string()
                        .replace(&task_id_subscript, "{t}");
                    let ident = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".into()));
                    let mut kw = LinkedHashMap::new();
                    kw.insert("t".into(), ident);
                    let replacement = AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(StringLiteral::new_wrapped(
                            replaced.as_str().into(),
                            false,
                        )),
                        kw,
                    ));
                    full_task_ids
                        .entry(replacement)
                        .or_insert(HashSet::new())
                        .insert(t.task_id.clone());

                    if let AOption(ROption::RSome(ref p)) = t.params {
                        for (key, val) in p.kwargs.iter() {
                            let val_no_ancestors = val.clone_without_ancestors();
                            if let AST::StringLiteral(rw) = val {
                                let x = rw.read();
                                if x.value().as_str() == task_id_subscript.as_str() {
                                    // TODO: pass this to ForLoopETLFlow
                                    let ident = AST::SimpleIdentifier(
                                        SimpleIdentifier::new_wrapped("t".into()),
                                    );
                                    kwargs_by_task_id
                                        .entry((key.clone(), ident))
                                        .or_insert(HashSet::new())
                                        .insert(t.task_id.clone());
                                } else {
                                    let val = x.value().as_str().to_string();
                                    let replaced = val.replace(&task_id_subscript, "{t}");
                                    if replaced != val {
                                        let ident = AST::SimpleIdentifier(
                                            SimpleIdentifier::new_wrapped("t".into()),
                                        );
                                        let mut kw = LinkedHashMap::new();
                                        kw.insert("t".into(), ident);
                                        let replacement = AST::Formatted(Formatted::new_wrapped(
                                            AST::StringLiteral(StringLiteral::new_wrapped(
                                                replaced.as_str().into(),
                                                false,
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
                    .collect::<LinkedHashMap<AString, AST>>();

                for (key, val) in compressible_kwargs_by_task_id.iter() {
                    compressible_kwargs.insert(key.clone(), val.clone());
                }

                for t in maybe_uncompressible.iter_mut() {
                    let mut new_deps = AVec::new();
                    for dep in t.deps.iter() {
                        if !compressible_deps.contains(dep) {
                            new_deps.push(dep.clone());
                        }
                    }
                    if let AOption(ROption::RSome(ref mut p)) = t.params {
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
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("params".into())),
                            AST::StringLiteral(StringLiteral::new_wrapped("task_id".into(), false)),
                            false,
                        )),
                        true,
                    ),
                };

                trace!(
                    "There are now {} maybe_uncompressible tasks",
                    maybe_uncompressible.len()
                );
                let compressed_task = ForLoopPythonBasedTask::new(
                    params_constraint,
                    compression_key,
                    maybe_uncompressible,
                    task_id,
                    insert_task_name,
                    render_dependencies,
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
