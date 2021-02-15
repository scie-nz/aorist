use crate::endpoints::EndpointConfig;
use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    BashPythonTask, Call, ConstantPythonTask, Formatted,
    Import, PrestoPythonTask, RPythonTask, SimpleIdentifier, StringLiteral, AST, NativePythonTask,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PythonSingleton {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
    endpoints: EndpointConfig,
}
impl ETLSingleton for PythonSingleton {
    fn get_preamble(&self) -> Vec<String> {
        let preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![p.clone()],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }
    // TODO: this should become a simple self.task_node.get_imports() call
    fn get_imports(&self) -> Vec<Import> {
        if let Some(Dialect::Presto(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            let presto_endpoints = self.endpoints.presto.as_ref().unwrap().clone();
            return PrestoPythonTask::new(command, self.task_val.clone(), presto_endpoints)
                .get_imports();
        } else if let Some(Dialect::Bash(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            return BashPythonTask::new(command, self.task_val.clone()).get_imports();
        } else if let Some(Dialect::R(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            return RPythonTask::new(command, self.task_val.clone()).get_imports();
        } else if self.dialect.is_none() {
            return ConstantPythonTask::new(
                AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string())),
                self.task_val.clone(),
            )
            .get_imports();
        }
        NativePythonTask::new(
            vec![AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.args.clone(),
                self.kwargs.clone(),
            ))],
            // TODO: add imports from preamble
            Vec::new(),
            self.task_val.clone(),
        )
        .get_imports()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    // TODO: this should become a simple self.task_node.get_statements() call
    fn get_statements(&self) -> Vec<AST> {
        if let Some(Dialect::Presto(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            let presto_endpoints = self.endpoints.presto.as_ref().unwrap().clone();
            return PrestoPythonTask::new(command, self.task_val.clone(), presto_endpoints)
                .get_statements();
        } else if let Some(Dialect::Bash(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            return BashPythonTask::new(command, self.task_val.clone()).get_statements();
        } else if let Some(Dialect::R(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            return RPythonTask::new(command, self.task_val.clone()).get_statements();
        } else if self.dialect.is_none() {
            return ConstantPythonTask::new(
                AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string())),
                self.task_val.clone(),
            )
            .get_statements();
        }
        NativePythonTask::new(
            vec![AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.args.clone(),
                self.kwargs.clone(),
            ))],
            // TODO: add imports from preamble
            Vec::new(),
            self.task_val.clone(),
        )
        .get_statements()
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
        }
    }
    fn get_type() -> String {
        "python".to_string()
    }
}
pub struct PythonDAG {}
impl ETLDAG for PythonDAG {
    type T = PythonSingleton;
    fn new() -> Self {
        Self {}
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        _py: Python<'a>,
        statements: Vec<&'a PyAny>,
        _ast_module: &'a PyModule,
    ) -> Vec<&'a PyAny> {
        statements
    }
}
