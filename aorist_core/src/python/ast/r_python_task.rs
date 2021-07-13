use crate::python::PythonImport;
use aorist_ast::{Assignment, Attribute, Call, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    RPythonTask,
    |task: &RPythonTask| vec![task.call.clone()],
    |task: &RPythonTask| {
        vec![AST::Assignment(Assignment::new_wrapped(
            task.task_val.clone(),
            task.call.clone(),
        ))]
    },
    |_task: &RPythonTask| {
        vec![
            PythonImport::PythonModuleImport("subprocess".to_string(), None),
            PythonImport::PythonModuleImport("rpy2.objects".to_string(), Some("robjects".to_string())),
        ]
    },
    PythonImport,
    r_script: Option<AST>,
    call: AST,
    task_val: AST,
);
