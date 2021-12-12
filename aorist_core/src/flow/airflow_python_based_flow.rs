use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::python_based_flow::PythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    BashPythonTask, ConstantPythonTask, NativePythonPreamble, NativePythonTask, PrestoPythonTask,
    PythonFlowBuilderInput, PythonImport, PythonPreamble, PythonTask, RPythonTask,
};
use abi_stable::std_types::ROption;
use aorist_ast::{
    Assignment, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Expression, Formatted, List,
    None, SimpleIdentifier, StringLiteral, AST,
};
use aorist_primitives::AOption;
use aorist_primitives::TPrestoEndpoints;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::marker::PhantomData;

#[derive(Clone, Hash, PartialEq)]
pub struct AirflowPythonBasedFlow<U: AoristUniverse>
where
    U::TEndpoints: TPrestoEndpoints,
{
    task_id: AST,
    task_val: AST,
    command: AOption<AString>,
    args: AVec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: AOption<AST>,
    preamble: AOption<AString>,
    dialect: AOption<Dialect>,
    endpoints: U::TEndpoints,
    node: PythonTask,
    _universe: PhantomData<U>,
}
impl<U: AoristUniverse> PythonBasedFlow<U> for AirflowPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> AOption<AString> {
        self.preamble.clone()
    }
}
impl<U: AoristUniverse> AirflowPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn compute_task_args(&self) -> AVec<AST> {
        AVec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        let mut kwargs;
        if self.dialect.is_none() {
            kwargs = self.kwargs.clone();
        } else {
            kwargs = LinkedHashMap::new();
            let call_param_name = match self.dialect {
                AOption(ROption::RSome(Dialect::Python(_)))
                | AOption(ROption::RSome(Dialect::R(_)))
                | AOption(ROption::RSome(Dialect::Presto(_)))
                | AOption(ROption::RNone) => "python_callable".into(),
                AOption(ROption::RSome(Dialect::Bash(_))) => "bash_command".into(),
            };
            // TODO: deprecate this once Bash tasks also migrate
            let call_param_value = match self.dialect {
                AOption(ROption::RSome(Dialect::Bash(_))) => {
                    AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(StringLiteral::new_wrapped(
                            self.command.as_ref().unwrap().clone(),
                            false,
                        )),
                        self.kwargs.clone(),
                    ))
                }
                _ => {
                    let call = self.node.get_call().unwrap();
                    match call {
                        AST::Call(call_rw) => call_rw.read().function(),
                        _ => panic!("AST object should be call"),
                    }
                }
            };
            kwargs.insert(call_param_name, call_param_value);
            if let AOption(ROption::RSome(Dialect::Python(_)))
            | AOption(ROption::RSome(Dialect::R(_)))
            | AOption(ROption::RSome(Dialect::Presto(_)))
            | AOption(ROption::RNone) = self.dialect
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
            AOption(ROption::RSome(Dialect::Python(_))) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            AOption(ROption::RSome(Dialect::Bash(_))) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("BashOperator".into()))
            }
            AOption(ROption::RSome(Dialect::Presto(_))) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            AOption(ROption::RSome(Dialect::R(_))) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
            }
            AOption(ROption::RNone) => {
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("DummyOperator".into()))
            }
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
    fn get_imports(&self) -> AVec<PythonImport> {
        match self.dialect {
            AOption(ROption::RSome(Dialect::Python(_)))
            | AOption(ROption::RSome(Dialect::R(_))) => vec![PythonImport::PythonFromImport(
                "airflow.operators.python_operator".into(),
                "PythonOperator".into(),
                AOption(ROption::RNone),
            )],
            AOption(ROption::RSome(Dialect::Bash(_)))
            | AOption(ROption::RSome(Dialect::Presto(_))) => {
                vec![PythonImport::PythonFromImport(
                    "airflow.operators.bash_operator".into(),
                    "BashOperator".into(),
                    AOption(ROption::RNone),
                )]
            }
            AOption(ROption::RNone) => vec![PythonImport::PythonFromImport(
                "airflow.operators.dummy_operator".into(),
                "DummyOperator".into(),
                AOption(ROption::RNone),
            )],
        }
        .into_iter()
        .collect()
    }
    fn get_preamble(&self) -> Result<AVec<PythonPreamble>, pyo3::PyErr> {
        // TODO: this should be deprecated
        let mut preambles = match self.dialect {
            AOption(ROption::RSome(Dialect::Python(_))) => match self.preamble {
                AOption(ROption::RSome(ref p)) => vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p.clone())?,
                )],
                AOption(ROption::RNone) => Vec::new(),
            },
            _ => Vec::new(),
        };
        if let AOption(ROption::RSome(p)) = self.node.get_preamble() {
            preambles.push(p)
        }
        Ok(preambles.into_iter().collect())
    }
    fn get_dialect(&self) -> AOption<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> AVec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AST::Assignment(Assignment::new_wrapped(
            self.task_val.clone(),
            creation_expr,
        ))];
        if let AOption(ROption::RSome(ref dependencies)) = self.dep_list {
            statements.push(AST::Expression(Expression::new_wrapped(AST::Call(
                Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".into(),
                        false,
                    )),
                    vec![dependencies.clone()].into_iter().collect(),
                    LinkedHashMap::new(),
                ),
            ))));
        }
        statements.into_iter().collect()
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
        endpoints: U::TEndpoints,
    ) -> Self {
        let command = match &dialect {
            AOption(ROption::RSome(Dialect::Presto(_))) => AST::StringLiteral(
                StringLiteral::new_wrapped(call.as_ref().unwrap().clone(), true),
            ),
            AOption(ROption::RSome(_)) => AST::StringLiteral(StringLiteral::new_wrapped(
                call.as_ref().unwrap().clone(),
                false,
            )),
            AOption(ROption::RNone) => {
                AST::StringLiteral(StringLiteral::new_wrapped("Done".into(), false))
            }
        };
        let node = match &dialect {
            AOption(ROption::RSome(Dialect::Presto(_))) => {
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
            AOption(ROption::RSome(Dialect::Bash(_))) => {
                PythonTask::BashPythonTask(BashPythonTask::new_wrapped(
                    command,
                    kwargs.clone(),
                    task_val.clone(),
                    dep_list.clone(),
                ))
            }
            AOption(ROption::RSome(Dialect::R(_))) => {
                PythonTask::RPythonTask(RPythonTask::new_wrapped(
                    task_val.clone(),
                    command,
                    args.clone(),
                    kwargs.clone(),
                    dep_list.clone(),
                    match preamble {
                        AOption(ROption::RSome(ref p)) => AOption(ROption::RSome(p.clone())),
                        AOption(ROption::RNone) => AOption(ROption::RNone),
                    },
                ))
            }
            AOption(ROption::RSome(Dialect::Python(_))) => {
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
            AOption(ROption::RNone) => PythonTask::ConstantPythonTask(
                ConstantPythonTask::new_wrapped(command, task_val.clone(), dep_list.clone()),
            ),
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
        mut statements: AVec<PythonFlowBuilderInput>,
        flow_name: AOption<AString>,
    ) -> AVec<PythonFlowBuilderInput> {
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
                ))]
                .into_iter()
                .collect(),
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
                ]
                .into_iter()
                .collect(),
                LinkedHashMap::new(),
            )),
        );
        kwargs.insert(
            "tags".into(),
            AST::List(List::new_wrapped(
                vec![AST::StringLiteral(StringLiteral::new_wrapped(
                    "aorist".into(),
                    false,
                ))]
                .into_iter()
                .collect(),
                false,
            )),
        );
        let dag_call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("DAG".into())),
            vec![AST::StringLiteral(StringLiteral::new_wrapped(
                match flow_name {
                    AOption(ROption::RSome(x)) => x,
                    AOption(ROption::RNone) => "flow".into(),
                },
                false,
            ))]
            .into_iter()
            .collect(),
            kwargs,
        ));
        let dag_call_assign = AST::Assignment(Assignment::new_wrapped(dag, dag_call));
        statements.insert(
            0,
            PythonFlowBuilderInput::statements_only(
                vec![default_args_assign, dag_call_assign]
                    .into_iter()
                    .collect(),
                "Setting up Airflow FlowBuilder".into(),
                AOption(ROption::RNone),
                AOption(ROption::RNone),
            ),
        );
        statements
    }
    fn get_flow_imports(&self) -> AVec<PythonImport> {
        vec![
            PythonImport::PythonFromImport("airflow".into(), "DAG".into(), AOption(ROption::RNone)),
            PythonImport::PythonFromImport(
                "datetime".into(),
                "datetime".into(),
                AOption(ROption::RNone),
            ),
        ]
        .into_iter()
        .collect()
    }
}
