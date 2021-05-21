#![allow(dead_code)]
use crate::endpoints::PrestoConfig;
use crate::python::PythonImport;
use aorist_ast::{
    Assignment, Attribute, BigIntLiteral, BooleanLiteral, Call, Dict, Expression, Formatted,
    SimpleIdentifier, StringLiteral, AST,
};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    PrestoPythonTask,
    |task: &PrestoPythonTask| vec![task.sql.clone()],
    |task: &PrestoPythonTask| {
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
                    false,
                )),
                AST::StringLiteral(StringLiteral::new_wrapped("''".to_string(), false)),
                command_ident.clone(),
            ],
            LinkedHashMap::new(),
        ));

        command_map.insert("command".to_string(), command_ident.clone());
        vec![
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
                        false,
                    )),
                    command_map,
                ))],
                LinkedHashMap::new(),
            )))),
            AST::Assignment(Assignment::new_wrapped(task.task_val.clone(), rows)),
        ]
    },
    |_task: &PrestoPythonTask| {
        vec![
            PythonImport::PythonModuleImport("subprocess".to_string(), None),
            PythonImport::PythonModuleImport("trino".to_string(), None),
            PythonImport::PythonModuleImport("re".to_string(), None),
        ]
    },
    PythonImport,
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
            AST::StringLiteral(StringLiteral::new_wrapped(
                presto_endpoints.server.clone(),
                false,
            )),
        );
        kwargs.insert(
            "user".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped(
                presto_endpoints.user.clone(),
                false,
            )),
        );
        kwargs.insert(
            "port".to_string(),
            AST::BigIntLiteral(BigIntLiteral::new_wrapped(presto_endpoints.httpPort as i64)),
        );
        kwargs.insert(
            "catalog".to_string(),
            AST::StringLiteral(StringLiteral::new_wrapped("hive".to_string(), false)),
        );
        let mut session_properties = LinkedHashMap::new();
        session_properties.insert(
            "redistribute_writes".to_string(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(false)),
        );
        kwargs.insert(
            "session_properties".to_string(),
            AST::Dict(Dict::new_wrapped(session_properties)),
        );

        AST::Assignment(Assignment::new_wrapped(
            self.connection_ident(),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("trino".to_string())),
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
