use crate::r::r_import::RImport;
use aorist_ast::{Assignment, Call, Expression, SimpleIdentifier, AST};
use aorist_util::{AString, AVec};
use aorist_primitives::{define_task_node};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;

define_task_node!(
    ConstantRTask,
    |task: &ConstantRTask| vec![task.name.clone()],
    |task: &ConstantRTask| {
        let call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".into())),
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
    |_task: &ConstantRTask| { vec![] },
    RImport,
    name: AST,
    task_val: AST,
);
