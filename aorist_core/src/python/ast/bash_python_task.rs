#![allow(dead_code)]
use super::airflow_bash_operator_task::AirflowBashOperatorTask;
use super::python_subprocess_task::PythonSubprocessTask;
use crate::python::ast::AirflowTaskBase;
use crate::python::ast::PythonTaskBase;
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{Formatted, AST};
use aorist_primitives::{define_task_node, AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;

define_task_node!(
    BashPythonTask,
    |task: &BashPythonTask| vec![task.call.clone()].into_iter().collect(),
    |task: &BashPythonTask| { task.get_subprocess_statements() },
    |task: &BashPythonTask| { task.get_python_subprocess_imports() },
    PythonImport,
    call: AST,
    kwargs: LinkedHashMap<AString, AST>,
    task_val: AST,
    dependencies: Option<AST>,
);
impl PythonTaskBase for BashPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl AirflowTaskBase for BashPythonTask {
    fn get_dependencies(&self) -> Option<AST> {
        self.dependencies.clone()
    }
}
impl PythonSubprocessTask for BashPythonTask {
    fn get_command(&self) -> AST {
        AST::Formatted(Formatted::new_wrapped(
            self.call.clone(),
            self.kwargs.clone(),
        ))
    }
}
impl AirflowBashOperatorTask for BashPythonTask {
    fn get_call_param_value(&self) -> AST {
        self.get_command()
    }
}
