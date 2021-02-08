use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    AoristStatement, AST, Attribute, Call, Formatted, Import, SimpleIdentifier, StringLiteral,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PrefectSingleton {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
    flow_identifier: AST,
}
impl ETLSingleton for PrefectSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) => vec![Import::FromImport(
                "prefect".to_string(),
                "task".to_string(),
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![Import::FromImport(
                    "prefect.tasks.shell".to_string(),
                    "ShellTask".to_string(),
                )]
            }
            None => vec![Import::FromImport(
                "prefect.tasks.core".to_string(),
                "Constant".to_string(),
            )],
        }
    }
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
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let task_creation = AoristStatement::Assign(self.get_task_val(), creation_expr);
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
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
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
                "flow".to_string(),
            )),
        }
    }
    fn compute_task_args(&self) -> Vec<AST> {
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.args.clone();
        }
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        if self.dialect.is_none() {
            return self.kwargs.clone();
        }
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.kwargs.clone();
        }
        let mut kwargs = LinkedHashMap::new();
        let call_param_name = match self.dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "command".to_string(),
            _ => panic!("Dialect not supported"),
        };
        let call_param_value = match self.dialect {
            Some(Dialect::Bash(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.kwargs.clone(),
            )),
            Some(Dialect::Presto(_)) => AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    format!("presto -e '{}'", self.command.as_ref().unwrap()).to_string(),
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
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(self.command.as_ref().unwrap().clone()),
            )),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(AST::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".to_string()),
            )),
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "Constant".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
    fn get_type() -> String {
        "prefect".to_string()
    }
}
impl PrefectSingleton {
    fn get_flow_identifier(&self) -> AST {
        self.flow_identifier.clone()
    }

    fn get_flow_add_edge_statement(&self, dep: AST) -> AoristStatement {
        let function = AST::Attribute(Attribute::new_wrapped(
            self.get_flow_identifier(),
            "add_edge".to_string(),
            false,
        ));
        let add_expr = AST::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val(), dep],
            LinkedHashMap::new(),
        ));
        AoristStatement::Expression(add_expr)
    }
    pub fn get_edge_addition_statements(&self) -> Vec<AoristStatement> {
        match self.dep_list {
            None => Vec::new(),
            Some(AST::List(_)) => {
                let target =
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("dep".to_string()));
                let for_stmt = AoristStatement::For(
                    target.clone(),
                    self.dep_list.as_ref().unwrap().clone(),
                    vec![self.get_flow_add_edge_statement(target.clone())],
                );
                vec![for_stmt]
            }
            _ => {
                let dep = self.dep_list.clone();
                let add_stmt = self.get_flow_add_edge_statement(dep.unwrap());
                vec![add_stmt]
            }
        }
    }
    pub fn get_flow_node_addition(&self) -> AoristStatement {
        let function = AST::Attribute(Attribute::new_wrapped(
            self.get_flow_identifier(),
            "add_node".to_string(),
            false,
        ));
        let add_expr = AST::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val()],
            LinkedHashMap::new(),
        ));
        AoristStatement::Expression(add_expr)
    }
}
pub struct PrefectDAG {
    flow_identifier: AST,
}
impl ETLDAG for PrefectDAG {
    type T = PrefectSingleton;
    fn new() -> Self {
        Self {
            flow_identifier: AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "flow".to_string(),
            )),
        }
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        py: Python<'a>,
        statements: Vec<&'a PyAny>,
        ast_module: &'a PyModule,
    ) -> Vec<&'a PyAny> {
        statements
            .into_iter()
            .chain(
                AoristStatement::Expression(AST::Call(Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.flow_identifier.clone(),
                        "run".into(),
                        false,
                    )),
                    Vec::new(),
                    LinkedHashMap::new(),
                )))
                .to_python_ast_node(py, ast_module)
                .into_iter(),
            )
            .collect()
    }
}
