use crate::dialect::Dialect;
use crate::flow::etl_flow::ETLFlow;
use crate::flow::flow_builder::FlowBuilderBase;
use crate::flow::python_based_flow::PythonBasedFlow;
use crate::flow::python_based_flow_builder::PythonBasedFlowBuilder;
use crate::python::{
    NativePythonPreamble, PythonFlowBuilderInput, PythonImport, PythonPreamble, RPythonTask,
};
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{
    Assignment, Attribute, Call, Expression, ForLoop, Formatted, SimpleIdentifier, StringLiteral,
    AST,
};
use aorist_primitives::register_task_nodes;
use aorist_primitives::{AString, AoristUniverse};
use aorist_primitives::TPrestoEndpoints;
use linked_hash_map::LinkedHashMap;
use pyo3::PyResult;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use crate::error::AoristError;

register_task_nodes! {
    PrefectTask,
    PythonImport,
    // This is the same as calling R from native Python
    RPythonTask,
}

#[derive(Clone, Hash, PartialEq)]
pub struct PrefectPythonBasedFlow<U: AoristUniverse> {
    task_id: AST,
    task_val: AST,
    command: Option<AString>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: Option<AST>,
    preamble: Option<AString>,
    dialect: Option<Dialect>,
    flow_identifier: AST,
    endpoints: U::TEndpoints,
    _universe: PhantomData<U>,
}

impl<U: AoristUniverse> PythonBasedFlow<U> for PrefectPythonBasedFlow<U>
where
    U::TEndpoints: TPrestoEndpoints,
{
    fn get_preamble_string(&self) -> Option<AString> {
        self.preamble.clone()
    }
}

impl<U: AoristUniverse> ETLFlow<U> for PrefectPythonBasedFlow<U> {
    type ImportType = PythonImport;
    type PreambleType = PythonPreamble;
    type ErrorType = pyo3::PyErr;

    fn get_preamble(&self) -> PyResult<Vec<PythonPreamble>> {
        let preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![PythonPreamble::NativePythonPreamble(
                    NativePythonPreamble::new(p.clone())?,
                )],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
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
        let task_creation =
            AST::Assignment(Assignment::new_wrapped(self.get_task_val(), creation_expr));
        let mut stmts = vec![task_creation];
        stmts.push(self.get_flow_node_addition());
        for stmt in self.get_edge_addition_statements() {
            stmts.push(stmt);
        }
        stmts
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
        Self {
            task_id,
            task_val,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
            flow_identifier: AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "flow".into(),
            )),
            endpoints,
            _universe: PhantomData,
        }
    }
    fn get_type() -> String {
        "prefect".into()
    }
    fn get_imports(&self) -> Vec<PythonImport> {
        match self.dialect {
            Some(Dialect::Python(_)) => vec![PythonImport::PythonFromImport(
                "prefect".into(),
                "task".into(),
                None,
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![PythonImport::PythonFromImport(
                    "prefect.tasks.shell".into(),
                    "ShellTask".into(),
                    None,
                )]
            }
            None => vec![PythonImport::PythonFromImport(
                "prefect.tasks.core".into(),
                "Constant".into(),
                None,
            )],
        }
    }
}
impl<U: AoristUniverse> PrefectPythonBasedFlow<U> {
    fn compute_task_args(&self) -> Vec<AST> {
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.args.clone();
        }
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        if self.dialect.is_none() {
            return self.kwargs.clone();
        }
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.kwargs.clone();
        }
        let mut kwargs = LinkedHashMap::new();
        let call_param_name = match self.dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "command".into(),
            _ => panic!("Dialect not supported"),
        };
        let call_param_value = match self.dialect {
            Some(Dialect::Bash(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                    false,
                )),
                self.kwargs.clone(),
            )),
            Some(Dialect::Presto(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    format!("presto -e '{}'", self.command.as_ref().unwrap()).as_str().into(),
                    true,
                )),
                self.kwargs.clone(),
            )),
            _ => panic!("Dialect not supported"),
        };
        kwargs.insert(call_param_name, call_param_value);
        kwargs
    }
    fn compute_task_call(&self) -> AST {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.command.as_ref().unwrap().clone(),
            ))),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".into()),
            )),
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "Constant".into(),
            ))),
            _ => Err(AoristError::OtherError(AString::from("Dialect not supported"))),
        }.unwrap()
    }
    fn get_flow_identifier(&self) -> AST {
        self.flow_identifier.clone()
    }
    fn get_flow_add_edge_statement(&self, dep: AST) -> AST {
        let function = AST::Attribute(Attribute::new_wrapped(
            self.get_flow_identifier(),
            "add_edge".into(),
            false,
        ));
        let add_expr = AST::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val(), dep],
            LinkedHashMap::new(),
        ));
        AST::Expression(Expression::new_wrapped(add_expr))
    }
    pub fn get_edge_addition_statements(&self) -> Vec<AST> {
        match self.dep_list {
            None => Vec::new(),
            Some(AST::List(_)) => {
                let target =
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dep".into()));
                let for_stmt = AST::ForLoop(ForLoop::new_wrapped(
                    target.clone(),
                    self.dep_list.as_ref().unwrap().clone(),
                    vec![self.get_flow_add_edge_statement(target.clone())],
                ));
                vec![for_stmt]
            }
            _ => {
                let dep = self.dep_list.clone();
                let add_stmt = self.get_flow_add_edge_statement(dep.unwrap());
                vec![add_stmt]
            }
        }
    }
    pub fn get_flow_node_addition(&self) -> AST {
        let function = AST::Attribute(Attribute::new_wrapped(
            self.get_flow_identifier(),
            "add_node".into(),
            false,
        ));
        let add_expr = AST::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val()],
            LinkedHashMap::new(),
        ));
        AST::Expression(Expression::new_wrapped(add_expr))
    }
}
pub struct PrefectFlowBuilder<U: AoristUniverse> {
    flow_identifier: AST,
    universe: PhantomData<U>,
}
impl<U: AoristUniverse> FlowBuilderBase<U> for PrefectFlowBuilder<U> {
    type T = PrefectPythonBasedFlow<U>;
    fn new() -> Self {
        Self {
            flow_identifier: AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "flow".into(),
            )),
            universe: PhantomData,
        }
    }
}
impl<U: AoristUniverse> PythonBasedFlowBuilder<U> for PrefectFlowBuilder<U> {
    fn get_flow_imports(&self) -> Vec<PythonImport> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn augment_statements(
        &self,
        statements: Vec<PythonFlowBuilderInput>,
        _flow_name: Option<AString>,
    ) -> Vec<PythonFlowBuilderInput> {
        // TODO: add flow definition
        statements
            .into_iter()
            .chain(
                vec![PythonFlowBuilderInput::statements_only(
                    vec![AST::Expression(Expression::new_wrapped(AST::Call(
                        Call::new_wrapped(
                            AST::Attribute(Attribute::new_wrapped(
                                self.flow_identifier.clone(),
                                "run".into(),
                                false,
                            )),
                            Vec::new(),
                            LinkedHashMap::new(),
                        ),
                    )))],
                    "Run Prefect flow".into(),
                    None,
                    None,
                )]
                .into_iter(),
            )
            .collect()
    }
}
