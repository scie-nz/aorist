#![allow(dead_code)]
use super::python_subprocess_task::PythonSubprocessTask;
use crate::python::ast::{Formatted, Import, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    BashPythonTask,
    |task: &BashPythonTask| vec![task.command.clone()]
        .into_iter()
        .chain(task.kwargs.values().map(|x| x.clone()))
        .collect(),
    |task: &BashPythonTask| { task.get_subprocess_statements() },
    |task: &BashPythonTask| { task.get_python_imports() },
    task_val: AST,
    command: AST,
    kwargs: LinkedHashMap<String, AST>,
);
impl PythonSubprocessTask for BashPythonTask {
    fn get_command(&self) -> AST {
        AST::Formatted(Formatted::new_wrapped(
            self.command.clone(),
            self.kwargs.clone(),
        ))
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
