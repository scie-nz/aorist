use crate::python::ast::{Assignment, Call, Expression, SimpleIdentifier, AST};
use crate::python::PythonImport;
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()],
    |task: &ConstantPythonTask| {
        let call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
            vec![task.name.clone()],
            LinkedHashMap::new(),
        ));

        vec![
            AST::Expression(Expression::new_wrapped(call)),
            AST::Assignment(Assignment::new_wrapped(
                task.task_val.clone(),
                task.name.clone(),
            )),
        ]
    },
    |_task: &ConstantPythonTask| { vec![] },
    name: AST,
    task_val: AST,
);
