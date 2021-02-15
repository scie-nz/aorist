#![allow(dead_code)]
use super::python_subprocess_task::PythonSubprocessTask;
use crate::python::ast::{Import, AST};
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    BashPythonTask,
    |task: &BashPythonTask| vec![task.command.clone()],
    |task: &BashPythonTask| { task.get_subprocess_statements() },
    |task: &BashPythonTask| { task.get_python_imports() },
    command: AST,
    task_val: AST,
);
impl PythonSubprocessTask for BashPythonTask {
    fn get_command(&self) -> AST {
        self.command.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
