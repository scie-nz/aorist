
use crate::r::r_import::RImport;
use aorist_ast::{Assignment, StringLiteral, AST};
use aorist_primitives::{AString, AVec, define_task_node};
use std::hash::Hash;
use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;

define_task_node!(
    NativeRTask,
    |task: &NativeRTask| task.statements.clone(),
    |task: &NativeRTask| {
        let mut statements: AVec<AST> = AVec::new();

        let mut it = task.statements.iter();

        let mut maybe_statement = it.next();
        let mut task_val_assigned = false;
        while let Some(statement) = maybe_statement {
            maybe_statement = it.next();
            statements.push(match statement {
                AST::Assignment(_) => statement.clone(),
                AST::Expression(expr) => match maybe_statement {
                    Some(_) => statement.clone(),
                    None => {
                        task_val_assigned = true;
                        AST::Assignment(Assignment::new_wrapped(
                            task.task_val.clone(),
                            expr.read().inner().clone(),
                        ))
                    }
                },
                _ => panic!(
                    "AST node of type {} found in NativePythonTask body",
                    statement.name()
                ),
            });
        }
        if !task_val_assigned {
            statements.push(AST::Assignment(Assignment::new_wrapped(
                task.task_val.clone(),
                AST::StringLiteral(StringLiteral::new_wrapped("Done".into(), false)),
            )));
        }
        statements
    },
    |task: &NativeRTask| task.imports.clone(),
    RImport,
    statements: AVec<AST>,
    imports: AVec<RImport>,
    task_val: AST,
);
