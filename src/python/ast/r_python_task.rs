use crate::python::ast::{Assignment, Attribute, Call, Import, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    RPythonTask,
    |task: &RPythonTask| vec![task.r_script.clone()],
    |task: &RPythonTask| {
        let call = AST::Call(Call::new_wrapped(
            AST::Attribute(Attribute::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("robjects".to_string())),
                "r".to_string(),
                false,
            )),
            vec![task.r_script.clone()],
            LinkedHashMap::new(),
        ));

        vec![AST::Assignment(Assignment::new_wrapped(
            task.task_val.clone(),
            call,
        ))]
    },
    |_task: &RPythonTask| {
        vec![
            Import::ModuleImport("subprocess".to_string(), None),
            Import::ModuleImport("rpy2".to_string(), None),
        ]
    },
    r_script: AST,
    task_val: AST,
);
