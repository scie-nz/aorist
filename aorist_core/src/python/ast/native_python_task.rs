use aorist_primitives::AVec;
use crate::python::ast::{AirflowTaskBase, PythonFunctionCallTask, PythonTaskBase};
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::AST;
use aorist_primitives::define_task_node;
use std::hash::Hash;

define_task_node!(
    NativePythonTask,
    |task: &NativePythonTask| vec![task.call.clone()],
    |task: &NativePythonTask| { task.get_native_python_statements() },
    |task: &NativePythonTask| task.imports.clone(),
    PythonImport,
    call: AST,
    imports: AVec<PythonImport>,
    task_val: AST,
    dep_list: Option<AST>,
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
    fn get_dependencies(&self) -> Option<AST> {
        self.dep_list.clone()
    }
}
