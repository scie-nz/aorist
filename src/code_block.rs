use crate::constraint::{
    ArgType, LiteralsMap, ParameterTuple, SimpleIdentifier, StringLiteral, Subscript,
};
use crate::constraint_state::ConstraintState;
use crate::etl_task::{ETLTask, ForLoopETLTask, StandaloneETLTask};
use crate::prefect::{
    PrefectConstantTaskRender, PrefectPythonTaskRender, PrefectRender, PrefectShellTaskRender,
};
use crate::prefect_singleton::PrefectSingleton;
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct CodeBlock<'a> {
    _dialect: Option<Dialect>,
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    task_render: PrefectRender<'a>,
    constraint_name: String,
}
impl<'a> CodeBlock<'a> {
    pub fn register_literals(&'a self, literals: LiteralsMap) {
        for member in &self.members {
            self.task_render
                .register_literals(literals.clone(), member.clone());
        }
    }
    fn get_constraints(&'a self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    pub fn new(
        dialect: Option<Dialect>,
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
    ) -> Self {
        let task_render = match dialect {
            Some(Dialect::Python(_)) => PrefectRender::Python(PrefectPythonTaskRender::new(
                members.clone(),
                constraint_name.clone(),
            )),
            Some(Dialect::Bash(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
                constraint_name.clone(),
            )),
            Some(Dialect::Presto(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
                constraint_name.clone(),
            )),
            None => PrefectRender::Constant(PrefectConstantTaskRender::new(
                members.clone(),
                constraint_name.clone(),
            )),
            _ => {
                panic!("Dialect not handled: {:?}", dialect.as_ref().unwrap());
            }
        };
        Self {
            _dialect: dialect,
            members,
            task_render,
            constraint_name,
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
    pub fn get_singletons(&'a self, literals: LiteralsMap) -> HashMap<String, PrefectSingleton> {
        self.set_task_vals();
        let tasks = self
            .get_constraints()
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                StandaloneETLTask::new(
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
        let mut etl_tasks: Vec<ETLTask> = Vec::new();

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

        let singletons = self
            .task_render
            .get_compressed_singletons(literals, self.constraint_name.clone());
        singletons
    }
}
