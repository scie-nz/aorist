use crate::python::PythonImport;
use aorist_ast::AST;
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use crate::python::ast::{PythonTaskBase, PythonStatementsTask, AirflowTaskBase};

define_task_node!(
    NativePythonTask,
    |task: &NativePythonTask| task.statements.clone(),
    |task: &NativePythonTask| {
        task.get_native_python_statements()
    },
    |task: &NativePythonTask| task.imports.clone(),
    PythonImport,
    statements: Vec<AST>,
    imports: Vec<PythonImport>,
    task_val: AST,
    dep_list: Option<AST>,
);
impl PythonTaskBase for NativePythonTask {
   fn get_task_val(&self) -> AST {
      self.task_val.clone()
   }
}
impl PythonStatementsTask for NativePythonTask {
   fn python_statements(&self) -> Vec<AST> {
        self.statements.clone()
   }
}
impl AirflowTaskBase for NativePythonTask {
    fn get_dependencies(&self) -> Option<AST> {
        self.dep_list.clone()        
    }
}
