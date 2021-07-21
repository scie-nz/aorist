use crate::python::PythonImport;
use aorist_ast::AST;
use aorist_primitives::{register_task_nodes};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

mod airflow_task_base;
mod airflow_bash_operator_task;
mod airflow_python_operator_task;
mod bash_python_task;
mod constant_python_task;
mod native_python_task;
mod presto_python_task;
mod python_subprocess_task;
mod python_function_call_task;
mod python_task_base;
mod r_python_task;

pub use airflow_task_base::AirflowTaskBase;
pub use airflow_python_operator_task::AirflowPythonOperatorTask;
pub use bash_python_task::BashPythonTask;
pub use constant_python_task::ConstantPythonTask;
pub use native_python_task::NativePythonTask;
pub use presto_python_task::PrestoPythonTask;
pub use r_python_task::RPythonTask;
pub use python_function_call_task::PythonFunctionCallTask;
pub use python_task_base::PythonTaskBase;

use crate::python::PythonPreamble;

register_task_nodes! {
    PythonTask,
    PythonImport,
    BashPythonTask,
    RPythonTask,
    NativePythonTask,
    ConstantPythonTask,
    PrestoPythonTask,
}

impl PythonTask {
    pub fn get_preamble(&self) -> Option<PythonPreamble> {
        let inner = match &self {
            PythonTask::BashPythonTask(_) => None,
            PythonTask::RPythonTask(x) => x.read().unwrap().get_preamble(),
            PythonTask::NativePythonTask(x) => x.read().unwrap().get_preamble(),
            PythonTask::ConstantPythonTask(x) => x.read().unwrap().get_preamble(),
            PythonTask::PrestoPythonTask(x) => x.read().unwrap().get_preamble(),
        };
        if let Some(p) = inner {
            return Some(PythonPreamble::NativePythonPreamble(p));
        }
        return None
    }
    pub fn get_call(&self) -> Option<AST> {
        match &self {
            PythonTask::BashPythonTask(_) => None,
            PythonTask::RPythonTask(x) => Some(x.read().unwrap().get_call()),
            PythonTask::NativePythonTask(x) => Some(x.read().unwrap().get_call()),
            PythonTask::ConstantPythonTask(x) => Some(x.read().unwrap().get_call()),
            PythonTask::PrestoPythonTask(x) => Some(x.read().unwrap().get_call()),
        }
    }
}
