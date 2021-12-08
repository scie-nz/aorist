
use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::python_based_flow::PythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    BashPythonTask, ConstantPythonTask, NativePythonTask, PrestoPythonTask, PythonImport,
    PythonPreamble, PythonTask, RPythonTask,
};
use aorist_ast::{Call, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::TPrestoEndpoints;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq)]
pub struct NativePythonBasedFlow<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    task_id: AST,
    task_val: AST,
    command: Option<AString>,
    args: AVec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: Option<AST>,
    preamble: Option<AString>,
    dialect: Option<Dialect>,
    endpoints: U::TEndpoints,
    node: PythonTask,
    _universe: PhantomData<U>,
}
impl<U: AoristUniverse> PythonBasedFlow<U> for NativePythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<AString> {
        self.preamble.clone()
    }
}

impl<U: AoristUniverse> ETLFlow<U> for NativePythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;
    type ErrorType = pyo3::PyErr;

    fn get_preamble(&self) -> pyo3::PyResult<AVec<PythonPreamble>> {
        // TODO: this should be deprecated
        let mut preambles = self.get_python_preamble()?;
        if let Some(p) = self.node.get_preamble() {
            preambles.push(p)
        }
        Ok(preambles)
    }
    fn get_imports(&self) -> AVec<PythonImport> {
        self.node.get_imports()
    }
    fn get_dialect(&self) -> Option<Dialect> {
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
        call: Option<AString>,
        args: AVec<AST>,
        kwargs: LinkedHashMap<AString, AST>,
        dep_list: Option<AST>,
        preamble: Option<AString>,
        dialect: Option<Dialect>,
        endpoints: U::TEndpoints,
    ) -> Self {
        let command = match &dialect {
            Some(Dialect::Presto(_)) => AST::StringLiteral(StringLiteral::new_wrapped(
                call.as_ref().unwrap().clone(),
                true,
            )),
            Some(_) => AST::StringLiteral(StringLiteral::new_wrapped(
                call.as_ref().unwrap().clone(),
                false,
            )),
            None => AST::StringLiteral(StringLiteral::new_wrapped("Done".into(), false)),
        };
        let node = match &dialect {
            Some(Dialect::Presto(_)) => {
                let presto_endpoints = endpoints.presto_config();
                PythonTask::PrestoPythonTask(PrestoPythonTask::new_wrapped(
                    command,
                    kwargs
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                match *v {
                                    AST::StringLiteral(ref x) => {
                                        if x.read().len() == 0 {
                                            panic!("Cannot process empty string for key: {}", k);
                                        }
                                        AST::StringLiteral(StringLiteral::new_wrapped(
                                            x.read().value().clone(),
                                            true,
                                        ))
                                    }
                                    _ => v.clone(),
                                },
                            )
                        })
                        .collect(),
                    task_val.clone(),
                    presto_endpoints,
                    dep_list.clone(),
                ))
            }
            Some(Dialect::Bash(_)) => PythonTask::BashPythonTask(BashPythonTask::new_wrapped(
                command,
                kwargs.clone(),
                task_val.clone(),
                dep_list.clone(),
            )),
            Some(Dialect::R(_)) => PythonTask::RPythonTask(RPythonTask::new_wrapped(
                task_val.clone(),
                command,
                args.clone(),
                kwargs.clone(),
                dep_list.clone(),
                preamble.clone(),
            )),
            Some(Dialect::Python(_)) => {
                PythonTask::NativePythonTask(NativePythonTask::new_wrapped(
                    AST::Call(Call::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                            call.as_ref().unwrap().clone(),
                        )),
                        args.clone(),
                        kwargs.clone(),
                    )),
                    // TODO: add imports from preamble
                    AVec::new(),
                    task_val.clone(),
                    dep_list.clone(),
                ))
            }
            None => PythonTask::ConstantPythonTask(ConstantPythonTask::new_wrapped(
                command,
                task_val.clone(),
                dep_list.clone(),
            )),
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
            _universe: PhantomData,
        }
    }
    fn get_type() -> String {
        "python".into()
    }
}
pub struct PythonFlowBuilder<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    universe: PhantomData<U>,
}
impl<U: AoristUniverse> FlowBuilderBase<U> for PythonFlowBuilder<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    type T = NativePythonBasedFlow<U>;
    fn new() -> Self {
        Self {
            universe: PhantomData,
        }
    }
}
impl<U: AoristUniverse> PythonBasedFlowBuilder<U> for PythonFlowBuilder<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_flow_imports(&self) -> AVec<PythonImport> {
        AVec::new()
    }
}
