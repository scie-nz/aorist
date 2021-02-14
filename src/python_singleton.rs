use crate::endpoints::EndpointConfig;
use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    Assignment, Attribute, BigIntLiteral, BooleanLiteral, Call, Expression, Formatted, Import,
    SimpleIdentifier, StringLiteral, Tuple, AST,
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
    conn_ident: Option<AST>,
    cursor_ident: Option<AST>,
}
impl PythonSingleton {
    // TODO: add endpoints
    fn presto_connection_statement(&self, endpoints: &EndpointConfig) -> AST {
        let mut kwargs = LinkedHashMap::new();
        let presto_endpoints = endpoints.presto.as_ref().unwrap();

        kwargs.insert(
            "host".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped(presto_endpoints.server.clone())),
        );
        kwargs.insert(
            "user".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped(presto_endpoints.user.clone())),
        );
        kwargs.insert(
            "port".to_string(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(presto_endpoints.httpPort as i64)),
        );
        kwargs.insert(
            "catalog".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped("hive".to_string())),
        );

        AST::Assignment(Assignment::new_wrapped(
            self.conn_ident.as_ref().unwrap().clone(),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                            "prestodb".to_string(),
                        )),
                        "dbapi".to_string(),
                        false,
                    )),
                    "connect".to_string(),
                    false,
                )),
                vec![],
                kwargs,
            )),
        ))
    }
    fn presto_cursor_statement(&self) -> AST {
        AST::Assignment(Assignment::new_wrapped(
            self.cursor_ident.as_ref().unwrap().clone(),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    self.conn_ident.as_ref().unwrap().clone(),
                    "cursor".to_string(),
                    false,
                )),
                vec![],
                LinkedHashMap::new(),
            )),
        ))
    }
}
impl ETLSingleton for PythonSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) | None => vec![],
            Some(Dialect::Bash(_)) | Some(Dialect::R(_)) => {
                vec![Import::ModuleImport("subprocess".to_string())]
            }
            Some(Dialect::Presto(_)) => vec![
                Import::ModuleImport("subprocess".to_string()),
                Import::ModuleImport("prestodb".to_string()),
                Import::ModuleImport("re".to_string()),
            ],
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
    fn get_statements(&self, endpoints: &EndpointConfig) -> Vec<AST> {
        if let Some(Dialect::Presto(_)) = self.dialect {
            let command = AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().to_string(),
                )),
                self.kwargs.clone(),
            ));
            let rows = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("rows".to_string()));
            let mut command_map = LinkedHashMap::new();
            let command_ident =
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("command".to_string()));
            let command_ident_with_comments = AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("re".to_string())),
                    "sub".to_string(),
                    false,
                )),
                vec![
                    AST::StringLiteral(StringLiteral::new_wrapped(
                        format!("'{}n{}s+'", "\\", "\\").to_string(),
                    )),
                    AST::StringLiteral(StringLiteral::new_wrapped("''".to_string())),
                    command_ident.clone(),
                ],
                LinkedHashMap::new(),
            ));

            command_map.insert("command".to_string(), command_ident.clone());
            return vec![
                self.presto_connection_statement(endpoints),
                self.presto_cursor_statement(),
                AST::Assignment(Assignment::new_wrapped(command_ident.clone(), command)),
                AST::Expression(Expression::new_wrapped(AST::Call(Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.cursor_ident.as_ref().unwrap().clone(),
                        "execute".to_string(),
                        false,
                    )),
                    vec![command_ident_with_comments],
                    LinkedHashMap::new(),
                )))),
                AST::Assignment(Assignment::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("rows".to_string())),
                    AST::Call(Call::new_wrapped(
                        AST::Attribute(Attribute::new_wrapped(
                            self.cursor_ident.as_ref().unwrap().clone(),
                            "fetchall".to_string(),
                            false,
                        )),
                        vec![],
                        LinkedHashMap::new(),
                    )),
                )),
                AST::Expression(Expression::new_wrapped(AST::Call(Call::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
                    vec![AST::Formatted(Formatted::new_wrapped(
                        AST::StringLiteral(StringLiteral::new_wrapped(
                            "Ran command: {command}".to_string(),
                        )),
                        command_map,
                    ))],
                    LinkedHashMap::new(),
                )))),
                AST::Assignment(Assignment::new_wrapped(self.get_task_val(), rows)),
            ];
        }
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        match self.dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                let process =
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("process".to_string()));
                let task_creation =
                    AST::Assignment(Assignment::new_wrapped(process.clone(), creation_expr));
                let task_assign = AST::Assignment(Assignment::new_wrapped(
                    AST::Tuple(Tuple::new_wrapped(
                        vec![
                            self.get_task_val().as_wrapped_assignment_target(),
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                "error".to_string(),
                            )),
                        ],
                        true,
                    )),
                    AST::Call(Call::new_wrapped(
                        AST::Attribute(Attribute::new_wrapped(
                            process,
                            "communicate".to_string(),
                            false,
                        )),
                        Vec::new(),
                        LinkedHashMap::new(),
                    )),
                ));
                vec![task_creation, task_assign]
            }
            _ => vec![AST::Assignment(Assignment::new_wrapped(
                self.get_task_val(),
                creation_expr,
            ))],
        }
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
            dialect: dialect.clone(),
            conn_ident: match &dialect {
                Some(Dialect::Presto(_)) => Some(AST::SimpleIdentifier(
                    SimpleIdentifier::new_wrapped("conn".to_string()),
                )),
                _ => None,
            },
            cursor_ident: match &dialect {
                Some(Dialect::Presto(_)) => Some(AST::SimpleIdentifier(
                    SimpleIdentifier::new_wrapped("cursor".to_string()),
                )),
                _ => None,
            },
        }
    }
    fn compute_task_args(&self) -> Vec<AST> {
        match self.dialect {
            Some(Dialect::Python(_)) => self.args.clone(),
            Some(Dialect::Bash(_)) => vec![AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.kwargs.clone(),
            ))],
            Some(Dialect::Presto(_)) => panic!("Should not be called for Presto"),
            None => vec![self.task_id.clone()],
            _ => panic!("Dialect not supported"),
        }
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        match self.dialect {
            Some(Dialect::Python(_)) => self.kwargs.clone(),
            None => LinkedHashMap::new(),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => {
                let mut kwargs = LinkedHashMap::new();
                kwargs.insert(
                    "stdout".to_string(),
                    AST::Attribute(Attribute::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                            "subprocess".to_string(),
                        )),
                        "PIPE".to_string(),
                        false,
                    )),
                );
                kwargs.insert(
                    "shell".to_string(),
                    AST::BooleanLiteral(BooleanLiteral::new_wrapped(true)),
                );
                kwargs
            }
            _ => panic!("Dialect not supported"),
        }
    }
    fn compute_task_call(&self) -> AST {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.command.as_ref().unwrap().clone(),
            ))),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => {
                Ok(AST::Attribute(Attribute::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".to_string())),
                    "Popen".to_string(),
                    false,
                )))
            }
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "print".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
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
