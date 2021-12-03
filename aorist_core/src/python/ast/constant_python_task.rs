use crate::python::ast::{AirflowTaskBase, PythonFunctionCallTask, PythonTaskBase};
use crate::python::PythonImport;
use aorist_ast::{Call, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use abi_stable::std_types::RArc;
use std::sync::RwLock;

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()],
    |task: &ConstantPythonTask| { task.get_native_python_statements() },
    |_task: &ConstantPythonTask| { vec![] },
    PythonImport,
    name: AST,
    task_val: AST,
    dep_list: Option<AST>,
);
impl AirflowTaskBase for ConstantPythonTask {
    fn get_dependencies(&self) -> Option<AST> {
        self.dep_list.clone()
    }
}
impl PythonTaskBase for ConstantPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl PythonFunctionCallTask for ConstantPythonTask {
    fn get_call(&self) -> AST {
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
            vec![self.name.clone()],
            LinkedHashMap::new(),
        ))
    }
}
