use crate::python::ast::{AirflowTaskBase, PythonFunctionCallTask, PythonTaskBase};
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{Call, SimpleIdentifier, AST};
use aorist_util::{AOption, AVec};
use aorist_primitives::{define_task_node};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()].into_iter().collect(),
    |task: &ConstantPythonTask| { task.get_native_python_statements() },
    |_task: &ConstantPythonTask| { vec![].into_iter().collect() },
    PythonImport,
    name: AST,
    task_val: AST,
    dep_list: AOption<AST>,
);
impl AirflowTaskBase for ConstantPythonTask {
    fn get_dependencies(&self) -> AOption<AST> {
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
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".into())),
            vec![self.name.clone()].into_iter().collect(),
            LinkedHashMap::new(),
        ))
    }
}
