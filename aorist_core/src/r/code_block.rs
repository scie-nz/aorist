use aorist_primitives::AVec;
use crate::code::{CodeBlock, CodeBlockWithForLoopCompression};
use crate::constraint::SatisfiableOuterConstraint;
use crate::endpoints::EndpointConfig;
use crate::flow::{CompressibleTask, ETLFlow, ETLTask, ForLoopCompressedTask};
use crate::parameter_tuple::ParameterTuple;
use crate::r::preamble::RPreamble;
use crate::r::r_import::RImport;
use crate::r::task::{ForLoopRBasedTask, RBasedTask};
use aorist_ast::{Formatted, SimpleIdentifier, StringLiteral, Subscript, AST};
use aorist_primitives::OuterConstraint;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::marker::PhantomData;
use tracing::trace;
use uuid::Uuid;

pub struct RBasedCodeBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a: 'b,
{
    tasks_dict: Option<AST>,
    task_identifiers: HashMap<Uuid, AST>,
    tasks: AVec<RBasedTask<T>>,
    params: HashMap<AString, Option<ParameterTuple>>,
    _lt: PhantomData<&'a ()>,
    _lt2: PhantomData<&'b ()>,
    _constraint: PhantomData<C>,
}
impl<'a, 'b, T, C> CodeBlock<'a, 'b, T, C> for RBasedCodeBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a: 'b,
{
    type P = RPreamble;
    type E = RBasedTask<T>;

    fn construct(
        tasks_dict: Option<AST>,
        tasks: AVec<Self::E>,
        task_identifiers: HashMap<Uuid, AST>,
        params: HashMap<AString, Option<ParameterTuple>>,
    ) -> Self {
        Self {
            tasks_dict,
            tasks,
            task_identifiers,
            params,
            _lt: PhantomData,
            _lt2: PhantomData,
            _constraint: PhantomData,
        }
    }

    fn get_statements(
        &self,
        endpoints: &EndpointConfig,
    ) -> (AVec<AST>, LinkedHashSet<RPreamble>, BTreeSet<RImport>) {
        let preambles_and_statements = self
            .tasks
            .iter()
            .map(|x| x.get_statements(endpoints))
            .collect::<AVec<_>>();
        // TODO: get_statements should run for members of self.tasks
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<RPreamble>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<RImport>>();
        let statements = preambles_and_statements
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .collect::<AVec<_>>();
        (statements, preambles, imports)
    }
    fn get_tasks_dict(&self) -> Option<AST> {
        self.tasks_dict.clone()
    }

    fn get_identifiers(&self) -> HashMap<Uuid, AST> {
        self.task_identifiers.clone()
    }

    fn get_params(&self) -> HashMap<AString, Option<ParameterTuple>> {
        self.params.clone()
    }
}
impl<'a, 'b, T, C> CodeBlockWithForLoopCompression<'a, 'b, T, C> for RBasedCodeBlock<'a, 'b, T, C>
where
    T: ETLFlow<ImportType = RImport, PreambleType = RPreamble>,
    C: OuterConstraint<'a, 'b> + SatisfiableOuterConstraint<'a, 'b>,
    'a: 'b,
{
    // TODO: push to trait
    fn run_task_compressions(
        compressible: LinkedHashMap<
            <<Self::E as ETLTask<T>>::S as CompressibleTask>::KeyType,
            AVec<<Self::E as ETLTask<T>>::S>,
        >,
        python_based_tasks: &mut AVec<Self::E>,
        constraint_name: AString,
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
                    .collect::<AVec<_>>();

                let mut deps: HashMap<AST, HashSet<AString>> = HashMap::new();
                let mut kwargs: LinkedHashMap<AString, HashMap<AST, HashSet<AString>>> =
                    LinkedHashMap::new();
                let mut kwargs_by_task_id: LinkedHashMap<(AString, AST), HashSet<AString>> =
                    LinkedHashMap::new();
                let mut full_task_ids: LinkedHashMap<AST, HashSet<AString>> = LinkedHashMap::new();

                for t in &maybe_uncompressible {
                    for dep in &t.deps {
                        deps.entry(dep.clone())
                            .or_insert(HashSet::new())
                            .insert(t.task_id.clone());
                    }
                    let task_id_subscript = t.task_id.split("__").last().unwrap().to_string();
                    let replaced = t.task_id.replace(&task_id_subscript, "{t}");
                    let ident =
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("t".into()));
                    let mut kw = LinkedHashMap::new();
                    kw.insert("t".into(), ident);
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
                                let x = rw.read();
                                if x.value() == task_id_subscript {
                                    // TODO: pass this to ForLoopETLFlow
                                    let ident = AST::SimpleIdentifier(
                                        SimpleIdentifier::new_wrapped("t".into()),
                                    );
                                    kwargs_by_task_id
                                        .entry((key.clone(), ident))
                                        .or_insert(HashSet::new())
                                        .insert(t.task_id.clone());
                                } else {
                                    let replaced = x.value().replace(&task_id_subscript, "{t}");
                                    if replaced != x.value() {
                                        let ident = AST::SimpleIdentifier(
                                            SimpleIdentifier::new_wrapped("t".into()),
                                        );
                                        let mut kw = LinkedHashMap::new();
                                        kw.insert("t".into(), ident);
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
                                "task_id".into(),
                                false,
                            )),
                            false,
                        )),
                        true,
                    ),
                };

                let compressed_task = ForLoopRBasedTask::new(
                    params_constraint,
                    compression_key,
                    maybe_uncompressible,
                    task_id,
                    insert_task_name,
                );
                python_based_tasks.push(RBasedTask::ForLoopRBasedTask(compressed_task));
            } else {
                for task in tasks.into_iter() {
                    python_based_tasks.push(RBasedTask::StandaloneRBasedTask(task));
                }
            }
        }
    }
}
