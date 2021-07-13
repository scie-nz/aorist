#![allow(dead_code)]
use super::airflow_bash_operator_task::AirflowBashOperatorTask;
use super::python_subprocess_task::PythonSubprocessTask;
use crate::python::PythonImport;
use aorist_ast::AST;
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    BashPythonTask,
    |task: &BashPythonTask| vec![task.command.clone()],
    |task: &BashPythonTask| { task.get_subprocess_statements() },
    |task: &BashPythonTask| { task.get_python_subprocess_imports() },
    PythonImport,
    command: AST,
    task_val: AST,
    dependencies: Option<AST>,
);
impl PythonSubprocessTask for BashPythonTask {
    fn get_command(&self) -> AST {
        self.command.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl AirflowBashOperatorTask for BashPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_dependencies(&self) -> Option<AST> {
        self.dependencies.clone()        
    }
    fn get_call_param_value(&self) -> AST {
        self.command.clone()
    }
}
