use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::flow_builder_input::FlowBuilderInput;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    BashPythonTask, ConstantPythonTask, NativePythonTask, PrestoPythonTask, PythonImport,
    PythonPreamble, RPythonTask, PythonTask, PythonFlowBuilderInput,
};
use aorist_ast::{Call, Expression, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::AoristUniverse;
use aorist_primitives::{TPrestoEndpoints};
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::hash::{Hash};
use std::marker::PhantomData;
use crate::flow::python_based_flow::PythonBasedFlow;

#[derive(Clone, Hash, PartialEq)]
pub struct NativePythonBasedFlow<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
    endpoints: U::TEndpoints,
    node: PythonTask,
    _universe: PhantomData<U>,
}
impl<U: AoristUniverse> PythonBasedFlow<U> for NativePythonBasedFlow<U> 
where
    U::TEndpoints: TPrestoEndpoints {
    fn get_preamble_string(&self) -> Option<String> {
        self.preamble.clone()
    }
}

impl<U: AoristUniverse> ETLFlow<U> for NativePythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;

    fn get_preamble(&self) -> Vec<PythonPreamble> {
        self.get_python_preamble()
    }
    fn get_imports(&self) -> Vec<PythonImport> {
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
        endpoints: U::TEndpoints,
    ) -> Self {
        let command = match &dialect {
            Some(_) => AST::StringLiteral(StringLiteral::new_wrapped(
                call.as_ref().unwrap().to_string(),
                false,
            )),
            None => AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string(), false)),
        };
        let node = match &dialect {
            Some(Dialect::Presto(_)) => {
                let presto_endpoints = endpoints.presto_config();
                PythonTask::PrestoPythonTask(PrestoPythonTask::new_wrapped(
                    command,
                    kwargs.clone(),
                    task_val.clone(),
                    presto_endpoints,
                    dep_list.clone(),
                ))
            }
            Some(Dialect::Bash(_)) => {
                PythonTask::BashPythonTask(BashPythonTask::new_wrapped(
                    command,
                    kwargs.clone(),
                    task_val.clone(),
                    dep_list.clone()
                ))
            }
            Some(Dialect::R(_)) => {
                PythonTask::RPythonTask(RPythonTask::new_wrapped(
                    task_val.clone(),
                    command,
                    args.clone(),
                    kwargs.clone(),
                    dep_list.clone(),
                    preamble.clone(),
                ))
            }
            Some(Dialect::Python(_)) => {
                PythonTask::NativePythonTask(NativePythonTask::new_wrapped(
                    vec![AST::Expression(Expression::new_wrapped(
                        AST::Call(Call::new_wrapped(
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                call.as_ref().unwrap().clone(),
                            )),
                            args.clone(),
                            kwargs.clone(),
                        ))
                    ))],
                    // TODO: add imports from preamble
                    Vec::new(),
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
        "python".to_string()
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
    fn get_flow_imports(&self) -> Vec<PythonImport> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        py: Python<'a>,
        statements: Vec<PythonFlowBuilderInput>,
        ast_module: &'a PyModule,
    ) -> Vec<(String, Vec<&'a PyAny>)> {
        statements
            .into_iter()
            .map(|statement| {
                (
                    statement.get_block_comment(),
                    statement.to_python_ast_nodes(py, ast_module, 0).unwrap(),
                )
            })
            .collect()
    }
}
