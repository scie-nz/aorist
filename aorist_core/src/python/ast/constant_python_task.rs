use crate::python::PythonImport;
use aorist_ast::{Assignment, Call, Expression, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()],
    |task: &ConstantPythonTask| { task.get_print_statements() },
    |_task: &ConstantPythonTask| { vec![] },
    PythonImport,
    name: AST,
    task_val: AST,
);
impl ConstantPythonTask {
   pub fn get_print_statements(&self) -> Vec<AST> {
        let call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
            vec![self.name.clone()],
            LinkedHashMap::new(),
        ));

        vec![
            AST::Expression(Expression::new_wrapped(call)),
            AST::Assignment(Assignment::new_wrapped(
                self.task_val.clone(),
                self.name.clone(),
            )),
        ]
   }
}
