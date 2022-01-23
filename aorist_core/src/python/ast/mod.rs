use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use aorist_ast::AST;
use aorist_primitives::register_task_nodes;
use aorist_util::{AOption, AVec};
use std::hash::{Hash, Hasher};

mod airflow_bash_operator_task;
mod airflow_python_operator_task;
mod airflow_task_base;
mod bash_python_task;
mod constant_python_task;
mod native_python_task;
mod presto_python_task;
mod python_function_call_task;
mod python_subprocess_task;
mod python_task_base;
mod r_python_task;

pub use airflow_python_operator_task::AirflowPythonOperatorTask;
pub use airflow_task_base::AirflowTaskBase;
pub use bash_python_task::BashPythonTask;
pub use constant_python_task::ConstantPythonTask;
pub use native_python_task::NativePythonTask;
pub use presto_python_task::PrestoPythonTask;
pub use python_function_call_task::PythonFunctionCallTask;
pub use python_task_base::PythonTaskBase;
pub use r_python_task::RPythonTask;

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
    pub fn get_preamble(&self) -> AOption<PythonPreamble> {
        let inner = match &self {
            PythonTask::BashPythonTask(_) => AOption(ROption::RNone),
            PythonTask::RPythonTask(x) => x.read().get_preamble(),
            PythonTask::NativePythonTask(x) => x.read().get_preamble(),
            PythonTask::ConstantPythonTask(x) => x.read().get_preamble(),
            PythonTask::PrestoPythonTask(x) => x.read().get_preamble(),
        };
        if let AOption(ROption::RSome(p)) = inner {
            return AOption(ROption::RSome(PythonPreamble::NativePythonPreamble(p)));
        }
        return AOption(ROption::RNone);
    }
    pub fn get_call(&self) -> AOption<AST> {
        match &self {
            PythonTask::BashPythonTask(_) => AOption(ROption::RNone),
            PythonTask::RPythonTask(x) => AOption(ROption::RSome(x.read().get_call())),
            PythonTask::NativePythonTask(x) => AOption(ROption::RSome(x.read().get_call())),
            PythonTask::ConstantPythonTask(x) => AOption(ROption::RSome(x.read().get_call())),
            PythonTask::PrestoPythonTask(x) => AOption(ROption::RSome(x.read().get_call())),
        }
    }
}
