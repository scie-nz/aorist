use crate::python::PythonImport;
use aorist_ast::AST;
use aorist_primitives::{register_task_nodes};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

mod airflow_task_base;
mod airflow_bash_operator_task;
mod bash_python_task;
mod constant_python_task;
mod native_python_task;
mod presto_python_task;
mod python_subprocess_task;
mod python_statements_task;
mod python_task_base;
mod r_python_task;

pub use airflow_task_base::AirflowTaskBase;
pub use bash_python_task::BashPythonTask;
pub use constant_python_task::ConstantPythonTask;
pub use native_python_task::NativePythonTask;
pub use presto_python_task::PrestoPythonTask;
pub use r_python_task::RPythonTask;
pub use python_statements_task::PythonStatementsTask;
pub use python_task_base::PythonTaskBase;

register_task_nodes! {
    PythonTask,
    PythonImport,
    BashPythonTask,
    RPythonTask,
    NativePythonTask,
    ConstantPythonTask,
    PrestoPythonTask,
}
