use aorist_util::AOption;
use abi_stable::std_types::ROption;
use aorist_primitives::AVec;
use crate::endpoints::EndpointConfig;
use crate::flow::etl_flow::ETLFlow;
use crate::r::{ConstantRTask, NativeRTask, RImport, RPreamble};
use aorist_ast::{Call, Expression, Formatted, SimpleIdentifier, StringLiteral, AST};
use aorist_core::Dialect;
use aorist_primitives::register_task_nodes;
use linked_hash_map::LinkedHashMap;
use std::hash::{Hash, Hasher};
use abi_stable::std_types::RArc;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;

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
    command: AOption<AString>,
    args: AVec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: AOption<AST>,
    preamble: AVec<RPreamble>,
    dialect: AOption<Dialect>,

    endpoints: EndpointConfig,
    node: RTask,
}

impl ETLFlow for NativeRBasedFlow {
    type ImportType = RImport;
    type PreambleType = RPreamble;

    fn get_preamble(&self) -> AVec<RPreamble> {
        self.preamble.clone()
    }

    fn get_imports(&self) -> AVec<RImport> {
        self.node.get_imports()
    }
    fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> AVec<AST> {
        self.node.get_statements()
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: AOption<AString>,
        args: AVec<AST>,
        kwargs: LinkedHashMap<AString, AST>,
        dep_list: AOption<AST>,
        preamble: AOption<AString>,
        dialect: AOption<Dialect>,
        endpoints: EndpointConfig,
    ) -> Self {
        let preambles = match dialect {
            Some(Dialect::R(_)) => match preamble {
                Some(ref p) => vec![RPreamble::new(p.clone())],
                None => AVec::new(),
            },
            Some(Dialect::Python(_)) => match preamble {
                Some(ref p) => vec![RPreamble::from_python(
                    call.as_ref().unwrap().clone(),
                    p.clone(),
                )],
                None => AVec::new(),
            },
            _ => AVec::new(),
        };
        let command = match &dialect {
            Some(Dialect::R(_)) => AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    call.as_ref().unwrap().clone(),
                )),
                args.clone(),
                kwargs.clone(),
            )),
            Some(Dialect::Python(_)) => {
                let call_with_args = AST::Call(Call::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                        call.as_ref().unwrap().clone(),
                    )),
                    args.clone(),
                    kwargs.clone(),
                ))
                .to_python_source();
                let mut kwargs = LinkedHashMap::new();
                kwargs.insert(
                    "sep".into(),
                    AST::StringLiteral(StringLiteral::new_wrapped("\n".into(), false)),
                );
                let code = AST::Call(Call::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("paste".into())),
                    vec![
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                            call.as_ref().unwrap().clone(),
                        )),
                        AST::StringLiteral(StringLiteral::new_wrapped(call_with_args, false)),
                    ],
                    kwargs,
                ));
                AST::Call(Call::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                        "py_run_string".into(),
                    )),
                    vec![code],
                    LinkedHashMap::new(),
                ))
            }
            Some(Dialect::Bash(_)) => AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("system".into())),
                vec![AST::Formatted(Formatted::new_wrapped(
                    AST::StringLiteral(StringLiteral::new_wrapped(
                        call.as_ref().unwrap().to_string(),
                        false,
                    )),
                    kwargs.clone(),
                ))],
                linked_hash_map::LinkedHashMap::new(),
            )),
            None => AST::StringLiteral(StringLiteral::new_wrapped("Done".into(), false)),
            _ => panic!("Dialect not supported"),
        };
        let node = match &dialect {
            Some(Dialect::R(_)) | Some(Dialect::Python(_)) | Some(Dialect::Bash(_)) => {
                RTask::NativeRTask(NativeRTask::new_wrapped(
                    vec![AST::Expression(Expression::new_wrapped(command))],
                    // TODO: add imports from preamble
                    AVec::new(),
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
            preamble: preambles,
            dialect: dialect.clone(),
            endpoints,
            node,
        }
    }
    fn get_type() -> String {
        "r".into()
    }
}
