use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::python_based_flow::PythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    BashPythonTask, ConstantPythonTask, NativePythonPreamble, NativePythonTask, PrestoPythonTask,
    PythonFlowBuilderInput, PythonImport, PythonPreamble, PythonTask, RPythonTask,
};
use aorist_ast::{
    Assignment, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Expression, Formatted, List,
    None, SimpleIdentifier, StringLiteral, AST,
};
use aorist_primitives::TPrestoEndpoints;
use aorist_primitives::{AString, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq)]
pub struct AirflowPythonBasedFlow<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    task_id: AST,
    task_val: AST,
    command: Option<AString>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: Option<AST>,
    preamble: Option<AString>,
    dialect: Option<Dialect>,
    endpoints: U::TEndpoints,
    node: PythonTask,
    _universe: PhantomData<U>,
}
impl<U: AoristUniverse> PythonBasedFlow<U> for AirflowPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<AString> {
        self.preamble.clone()
    }
}
impl<U: AoristUniverse> AirflowPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn compute_task_args(&self) -> Vec<AST> {
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        let mut kwargs;
        if self.dialect.is_none() {
            kwargs = self.kwargs.clone();
        } else {
            kwargs = LinkedHashMap::new();
            let call_param_name = match self.dialect {
                Some(Dialect::Python(_))
                | Some(Dialect::R(_))
                | Some(Dialect::Presto(_))
                | None => "python_callable".into(),
                Some(Dialect::Bash(_)) => "bash_command".into(),
            };
            // TODO: deprecate this once Bash tasks also migrate
            let call_param_value = match self.dialect {
                Some(Dialect::Bash(_)) => AST::Formatted(Formatted::new_wrapped(
                    AST::StringLiteral(StringLiteral::new_wrapped(
                        self.command.as_ref().unwrap().clone(),
                        false,
                    )),
                    self.kwargs.clone(),
                )),
                _ => {
                    let call = self.node.get_call().unwrap();
                    match call {
                        AST::Call(call_rw) => call_rw.read().function(),
                        _ => panic!("AST object should be call"),
                    }
                }
            };
            kwargs.insert(call_param_name, call_param_value);
            if let Some(Dialect::Python(_))
            | Some(Dialect::R(_))
            | Some(Dialect::Presto(_))
            | None = self.dialect
            {
                let call = self.node.get_call().unwrap();
                if let AST::Call(call_rw) = call {
                    let x = call_rw.read();
                    if x.args().len() > 0 {
                        panic!("Call should not have any arguments");
                    }
                    let inner_kwargs = x.keywords();
                    if inner_kwargs.len() > 0 {
                        kwargs.insert(
                            "op_kwargs".into(),
                            AST::Dict(Dict::new_wrapped(inner_kwargs)),
                        );
                    }
                } else {
                    panic!("AST object should be call");
                }
            }
        }
        kwargs.insert(
            "dag".into(),
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".into())),
        );
        kwargs.insert("task_id".into(), self.task_id.clone());
        kwargs
    }
    fn compute_task_call(&self) -> AST {
        match self.dialect {
            Some(Dialect::Python(_)) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            Some(Dialect::Bash(_)) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("BashOperator".into()))
            }
            Some(Dialect::Presto(_)) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            Some(Dialect::R(_)) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            None => AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("DummyOperator".into())),
        }
    }
}
impl<U: AoristUniverse> ETLFlow<U> for AirflowPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;
    type ErrorType = pyo3::PyErr;
    fn get_imports(&self) -> Vec<PythonImport> {
        match self.dialect {
            Some(Dialect::Python(_)) | Some(Dialect::R(_)) => vec![PythonImport::PythonFromImport(
                "airflow.operators.python_operator".into(),
                "PythonOperator".into(),
                None,
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => {
                vec![PythonImport::PythonFromImport(
                    "airflow.operators.bash_operator".into(),
                    "BashOperator".into(),
                    None,
                )]
            }
            None => vec![PythonImport::PythonFromImport(
                "airflow.operators.dummy_operator".into(),
                "DummyOperator".into(),
                None,
            )],
        }
    }
    fn get_preamble(&self) -> pyo3::PyResult<Vec<PythonPreamble>> {
        // TODO: this should be deprecated
        let mut preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p.clone())?,
                )],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        if let Some(p) = self.node.get_preamble() {
            preambles.push(p)
        }
        Ok(preambles)
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AST::Assignment(Assignment::new_wrapped(
            self.task_val.clone(),
            creation_expr,
        ))];
        if let Some(ref dependencies) = self.dep_list {
            statements.push(AST::Expression(Expression::new_wrapped(AST::Call(
                Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".into(),
                        false,
                    )),
                    vec![dependencies.clone()],
                    LinkedHashMap::new(),
                ),
            ))));
        }
        statements
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: Option<AString>,
        args: Vec<AST>,
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
                                    AST::StringLiteral(ref x) => AST::StringLiteral(
                                        StringLiteral::new_wrapped(x.read().value().clone(), true),
                                    ),
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
                match preamble {
                    Some(ref p) => Some(p.clone()),
                    None => None,
                },
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
            dialect,
            endpoints,
            node,
            _universe: PhantomData,
        }
    }
    fn get_type() -> String {
        "airflow".into()
    }
}

pub struct AirflowFlowBuilder<U: AoristUniverse> {
    universe: PhantomData<U>,
}
impl<U: AoristUniverse> FlowBuilderBase<U> for AirflowFlowBuilder<U>
where
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    type T = AirflowPythonBasedFlow<U>;
    fn new() -> Self {
        Self {
            universe: PhantomData,
        }
    }
}
impl<U: AoristUniverse> PythonBasedFlowBuilder<U> for AirflowFlowBuilder<U>
where
    <U as AoristUniverse>::TEndpoints: TPrestoEndpoints,
{
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn augment_statements(
        &self,
        mut statements: Vec<PythonFlowBuilderInput>,
        flow_name: Option<AString>,
    ) -> Vec<PythonFlowBuilderInput> {
        let default_args =
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("default_args".into()));
        let mut default_args_map: LinkedHashMap<AString, AST> = LinkedHashMap::new();
        default_args_map.insert(
            "owner".into(),
            AST::StringLiteral(StringLiteral::new_wrapped("airflow".into(), false)),
        );
        default_args_map.insert(
            "depends_on_past".into(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email".into(),
            AST::List(List::new_wrapped(
                vec![AST::StringLiteral(StringLiteral::new_wrapped(
                    "airflow@example.com".into(),
                    false,
                ))],
                false,
            )),
        );
        default_args_map.insert(
            "email_on_failure".into(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "email_on_retry".into(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        default_args_map.insert(
            "retries".into(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
        );
        default_args_map.insert(
            "retry_delay".into(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(300)),
        );

        let default_args_dict = AST::Dict(Dict::new_wrapped(default_args_map));
        let default_args_assign = AST::Assignment(Assignment::new_wrapped(
            default_args.clone(),
            default_args_dict,
        ));

        let dag = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".into()));

        let mut kwargs: LinkedHashMap<AString, AST> = LinkedHashMap::new();
        kwargs.insert("default_args".into(), default_args);
        kwargs.insert(
            "description".into(),
            AST::StringLiteral(StringLiteral::new_wrapped(
                "Auto-generated by Aorist".into(),
                false,
            )),
        );
        kwargs.insert("schedule_interval".into(), AST::None(None::new_wrapped()));
        kwargs.insert(
            "start_date".into(),
            AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("datetime".into())),
                vec![
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(2021)),
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
                    AST::BigIntLiteral(BigIntLiteral::new_wrapped(1)),
                ],
                LinkedHashMap::new(),
            )),
        );
        kwargs.insert(
            "tags".into(),
            AST::List(List::new_wrapped(
                vec![AST::StringLiteral(StringLiteral::new_wrapped(
                    "aorist".into(),
                    false,
                ))],
                false,
            )),
        );
        let dag_call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("DAG".into())),
            vec![AST::StringLiteral(StringLiteral::new_wrapped(
                match flow_name {
                    Some(x) => x,
                    None => "flow".into(),
                },
                false,
            ))],
            kwargs,
        ));
        let dag_call_assign = AST::Assignment(Assignment::new_wrapped(dag, dag_call));
        statements.insert(
            0,
            PythonFlowBuilderInput::statements_only(
                vec![default_args_assign, dag_call_assign],
                "Setting up Airflow FlowBuilder".into(),
                None,
                None,
            ),
        );
        statements
    }
    fn get_flow_imports(&self) -> Vec<PythonImport> {
        vec![
            PythonImport::PythonFromImport("airflow".into(), "DAG".into(), None),
            PythonImport::PythonFromImport("datetime".into(), "datetime".into(), None),
        ]
    }
}
