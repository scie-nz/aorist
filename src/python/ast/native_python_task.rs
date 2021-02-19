use crate::python::ast::{Assignment, Import, StringLiteral, AST};
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    NativePythonTask,
    |task: &NativePythonTask| task.statements.clone(),
    |task: &NativePythonTask| task
        .statements
        .clone()
        .into_iter()
        .map(|x| match &x {
            AST::Assignment(_) | AST::Expression(_) => x,
            _ => panic!(format!(
                "AST node of type {} found in NativePythonTask body",
                x.name()
            )),
        })
        .chain(
            vec![AST::Assignment(Assignment::new_wrapped(
                task.task_val.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string())),
            ))]
            .into_iter()
        )
        .collect(),
    |task: &NativePythonTask| task.imports.clone(),
    statements: Vec<AST>,
    imports: Vec<Import>,
    task_val: AST,
);
