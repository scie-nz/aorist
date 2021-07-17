use crate::python::PythonImport;
use aorist_ast::{Call, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use crate::python::ast::{PythonTaskBase, PythonStatementsTask};

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()],
    |task: &ConstantPythonTask| { task.get_native_python_statements() },
    |_task: &ConstantPythonTask| { vec![] },
    PythonImport,
    name: AST,
    task_val: AST,
);
impl PythonTaskBase for ConstantPythonTask {
   fn get_task_val(&self) -> AST {
      self.task_val.clone()
   }
}
impl PythonStatementsTask for ConstantPythonTask {
   fn python_statements(&self) -> Vec<AST> {
        let call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
            vec![self.name.clone()],
            LinkedHashMap::new(),
        ));

        vec![
            call,
            self.name.clone(),
        ]
   }
}
