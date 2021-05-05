use crate::python::AST;
use crate::r::r_import::RImport;
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    NativeRTask,
    |task: &NativeRTask| task.statements.clone(),
    |task: &NativeRTask| { task.statements.clone() },
    |task: &NativeRTask| task.imports.clone(),
    RImport,
    statements: Vec<AST>,
    imports: Vec<RImport>,
    task_val: AST,
);
