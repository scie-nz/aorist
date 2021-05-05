use crate::dialect::Dialect;
use crate::endpoints::EndpointConfig;
use crate::flow::etl_flow::ETLFlow;
use crate::python::{Call, Expression, SimpleIdentifier, StringLiteral, AST};
use crate::r::{ConstantRTask, NativeRTask, RImport};
use aorist_primitives::register_task_nodes;
use linked_hash_map::LinkedHashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

register_task_nodes! {
    RTask,
    RImport,
    ConstantRTask,
    NativeRTask,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct NativeRBasedFlow {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,

    endpoints: EndpointConfig,
    node: RTask,
}

impl ETLFlow for NativeRBasedFlow {
    type ImportType = RImport;

    fn get_preamble(&self) -> Vec<String> {
        let preambles = match self.dialect {
            Some(Dialect::R(_)) => match self.preamble {
                Some(ref p) => vec![p.clone()],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }

    fn get_imports(&self) -> Vec<RImport> {
        self.node.get_imports()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AST> {
        self.node.get_statements()
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        endpoints: EndpointConfig,
    ) -> Self {
        let command = match &dialect {
            Some(Dialect::R(_)) => AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    call.as_ref().unwrap().clone(),
                )),
                args.clone(),
                kwargs.clone(),
            )),
            None => AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string(), false)),
            _ => panic!("Dialect not supported"),
        };
        let node = match &dialect {
            Some(Dialect::R(_)) => {
                RTask::NativeRTask(NativeRTask::new_wrapped(
                    vec![AST::Expression(Expression::new_wrapped(command))],
                    // TODO: add imports from preamble
                    Vec::new(),
                    task_val.clone(),
                ))
            }
            None => RTask::ConstantRTask(ConstantRTask::new_wrapped(command, task_val.clone())),
            _ => panic!("Dialect not supported"),
        };

        Self {
            task_id,
            task_val,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect: dialect.clone(),
            endpoints,
            node,
        }
    }
    fn get_type() -> String {
        "r".to_string()
    }
}