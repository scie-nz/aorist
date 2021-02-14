#![allow(dead_code)]
use crate::endpoints::PrestoConfig;
use crate::python::ast::{
    Assignment, Attribute, BigIntLiteral, Call, SimpleIdentifier, StringLiteral, AST,
    Formatted, Expression,
};
use aorist_primitives::define_ast_node;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

pub trait TAssignmentTarget
where
    Self: Sized,
{
    fn as_assignment_target(&self) -> Self;
    fn as_wrapped_assignment_target(&self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self.as_assignment_target()))
    }
}

define_ast_node!(
    PrestoPythonTask,
    |task: &PrestoPythonTask| vec![task.sql.clone()],
    |task: &PrestoPythonTask, py: Python, ast_module: &'a PyModule| {
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
        let body = vec![
            task.presto_connection_statement(&task.endpoint),
            task.presto_cursor_statement(),
            AST::Assignment(Assignment::new_wrapped(
                command_ident.clone(),
                task.sql.clone(),
            )),
            AST::Expression(Expression::new_wrapped(AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    task.cursor_ident(),
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
                        task.cursor_ident(),
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
            AST::Assignment(Assignment::new_wrapped(task.task_val.clone(), rows)),
        ];
        let body_vec: Vec<&PyAny> = body
            .iter()
            .map(|x| x.to_python_ast_node(py, ast_module).unwrap())
            .collect();
        let body_list = PyList::new(py, body_vec);
        ast_module.call1("Module", (body_list.as_ref(),))
    },
    sql: AST,
    task_val: AST,
    endpoint: PrestoConfig,
);
impl PrestoPythonTask {
    fn cursor_ident(&self) -> AST {
        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("cursor".to_string()))
    }
    fn connection_ident(&self) -> AST {
        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("conn".to_string()))
    }
    fn presto_connection_statement(&self, presto_endpoints: &PrestoConfig) -> AST {
        let mut kwargs = LinkedHashMap::new();

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
            self.connection_ident(),
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
            self.cursor_ident(),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    self.connection_ident(),
                    "cursor".to_string(),
                    false,
                )),
                vec![],
                LinkedHashMap::new(),
            )),
        ))
    }
}
