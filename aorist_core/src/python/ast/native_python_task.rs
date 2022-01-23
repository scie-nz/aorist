use crate::python::ast::{AirflowTaskBase, PythonFunctionCallTask, PythonTaskBase};
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::AST;
use aorist_primitives::define_task_node;
use aorist_util::AOption;
use aorist_util::AVec;
use std::hash::Hash;

define_task_node!(
    NativePythonTask,
    |task: &NativePythonTask| vec![task.call.clone()].into_iter().collect(),
    |task: &NativePythonTask| { task.get_native_python_statements() },
    |task: &NativePythonTask| task.imports.clone(),
    PythonImport,
    call: AST,
    imports: AVec<PythonImport>,
    task_val: AST,
    dep_list: AOption<AST>,
);
impl PythonTaskBase for NativePythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl PythonFunctionCallTask for NativePythonTask {
    fn get_call(&self) -> AST {
        self.call.clone()
    }
}
impl AirflowTaskBase for NativePythonTask {
    fn get_dependencies(&self) -> AOption<AST> {
        self.dep_list.clone()
    }
}
