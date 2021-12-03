#![allow(dead_code)]
use crate::python::ast::AirflowTaskBase;
use crate::python::ast::{PythonFunctionCallTask, PythonTaskBase};
use crate::python::NativePythonPreamble;
use crate::python::PythonImport;
use aorist_ast::{Call, Formatted, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use aorist_primitives::PrestoConfig;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use abi_stable::std_types::RArc;
use std::sync::RwLock;

define_task_node!(
    PrestoPythonTask,
    |task: &PrestoPythonTask| vec![task.sql.clone()],
    |task: &PrestoPythonTask| { task.get_native_python_statements() },
    |_task: &PrestoPythonTask| {
        vec![
            PythonImport::PythonModuleImport("subprocess".to_string(), None),
            PythonImport::PythonModuleImport("trino".to_string(), None),
            PythonImport::PythonModuleImport("re".to_string(), None),
        ]
    },
    PythonImport,
    sql: AST,
    kwargs: LinkedHashMap<String, AST>,
    task_val: AST,
    endpoint: PrestoConfig,
    dependencies: Option<AST>,
);

impl PythonTaskBase for PrestoPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl PythonFunctionCallTask for PrestoPythonTask {
    fn get_preamble(&self) -> Option<NativePythonPreamble> {
        let re = PythonImport::PythonModuleImport("re".to_string(), None);
        let trino = PythonImport::PythonModuleImport("trino".to_string(), None);
        let body = format!(
            "
def execute_trino_sql(query):
    connection = trino.dbapi.connect(
        host='{host}',
        user='{user}',
        port={port},
        catalog='hive',
        session_properties={{
            'redistribute_writes': False,
        }}
    )
    if isinstance(query, list):
        for q in query:
            cursor = connection.cursor()
            cursor.execute(q)
            cursor.fetchall()
            print('Ran query: ' + chr(10) + ' ' + q)
    else:
        cursor = connection.cursor()
        cursor.execute(query)
        print('Ran query: ' + chr(10) + ' ' + query)
        cursor.fetchall()
",
            host = self.endpoint.server,
            user = self.endpoint.user,
            port = self.endpoint.http_port
        );
        Some(NativePythonPreamble {
            imports: vec![re, trino],
            from_imports: Vec::new(),
            body: body.to_string(),
        })
    }
    fn get_call(&self) -> AST {
        let query;
        if let AST::StringLiteral(ref s) = self.sql {
            if s.read().unwrap().value() == "{queries}" {
                query = self.kwargs.get("queries").unwrap().clone();
            } else {
                query = AST::Formatted(Formatted::new_wrapped(
                    self.sql.clone(),
                    self.kwargs.clone(),
                ));
            }
        } else {
            panic!("SQL should be StringLiteral.");
        }
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "execute_trino_sql".to_string(),
            )),
            vec![],
            vec![("query".to_string(), query)].into_iter().collect(),
        ))
    }
}
impl AirflowTaskBase for PrestoPythonTask {
    fn get_dependencies(&self) -> Option<AST> {
        self.dependencies.clone()
    }
}
