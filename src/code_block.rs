use crate::constraint::{
    AoristStatement, ArgType, Import, ParameterTuple, SimpleIdentifier, StringLiteral, Subscript,
};
use crate::constraint_state::ConstraintState;
use crate::etl_singleton::ETLSingleton;
use crate::etl_task::{ETLTask, ForLoopETLTask, StandaloneETLTask};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

pub struct CodeBlock<'a, T>
where
    T: ETLSingleton,
{
    _dialect: Option<Dialect>,
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    constraint_name: String,
    singleton_type: PhantomData<T>,
}
impl<'a, T> CodeBlock<'a, T>
where
    T: ETLSingleton,
{
    fn get_constraints(&'a self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    pub fn new(
        dialect: Option<Dialect>,
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
    ) -> Self {
        Self {
            _dialect: dialect,
            members,
            constraint_name,
            singleton_type: PhantomData,
        }
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                (x.get_task_name(), x.get_params())
            })
            .collect()
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    /// assigns task values (Python variables in which they will be stored)
    /// to each member of the code block.
    fn set_task_vals(&'a self) {
        let num_constraints = self.get_constraints().len();
        for rw in self.get_constraints() {
            let mut write = rw.write().unwrap();
            let name = write.get_task_name();
            // TODO: magic number
            if num_constraints <= 2 {
                write.set_task_val(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    name,
                )));
            } else {
                let shorter_name =
                    name.replace(&format!("{}__", self.get_constraint_name()).to_string(), "");

                write.set_task_val(ArgType::Subscript(Subscript::new_wrapped(
                    ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                        format!("tasks_{}", self.get_constraint_name()).to_string(),
                    )),
                    ArgType::StringLiteral(Arc::new(RwLock::new(StringLiteral::new(shorter_name)))),
                )));
            }
        }
    }
    pub fn get_statements(
        &'a self,
    ) -> (
        Vec<AoristStatement>,
        LinkedHashSet<String>,
        BTreeSet<Import>,
    ) {
        self.set_task_vals();
        let tasks = self
            .get_constraints()
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                StandaloneETLTask::new(
                    x.get_task_name(),
                    x.get_task_val(),
                    x.get_call(),
                    x.get_params(),
                    x.get_dependencies(),
                    x.get_preamble(),
                    x.get_dialect(),
                )
            })
            .collect::<Vec<_>>();

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
        let mut num_compression_tasks = 0;
        for (compression_key, tasks) in compressible.into_iter() {
            // TODO: this is a magic number
            if tasks.len() > 2 {
                let params_constraint = ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    match num_compression_tasks {
                        0 => format!("params_{}", self.get_constraint_name()).to_string(),
                        _ => format!(
                            "params_{}_{}",
                            self.get_constraint_name(),
                            num_compression_tasks + 1
                        )
                        .to_string(),
                    },
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
                num_compression_tasks += 1;
            } else {
                for task in tasks.into_iter() {
                    etl_tasks.push(ETLTask::StandaloneETLTask(task));
                }
            }
        }
        let preambles_and_statements = etl_tasks
            .into_iter()
            .map(|x| x.get_statements())
            .collect::<Vec<_>>();
        let preambles = preambles_and_statements
            .iter()
            .map(|x| x.1.clone().into_iter())
            .flatten()
            .collect::<LinkedHashSet<String>>();
        let imports = preambles_and_statements
            .iter()
            .map(|x| x.2.clone().into_iter())
            .flatten()
            .collect::<BTreeSet<Import>>();
        (
            preambles_and_statements
                .iter()
                .map(|x| x.0.clone())
                .flatten()
                .collect::<Vec<_>>(),
            preambles,
            imports,
        )
    }
}
