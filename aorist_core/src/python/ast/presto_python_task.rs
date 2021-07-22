#![allow(dead_code)]
use crate::python::PythonImport;
use aorist_ast::{
    Formatted, Call,
    SimpleIdentifier, AST,
};
use aorist_primitives::define_task_node;
use aorist_primitives::PrestoConfig;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use crate::python::ast::{PythonTaskBase, PythonFunctionCallTask};
use crate::python::ast::AirflowTaskBase;
use crate::python::NativePythonPreamble;

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
        let body = format!("
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
    cursor = connection.cursor()
    cursor.execute(query)
    print('Ran query:\\n %s' % query)
    return cursor.fetchall()
", host=self.endpoint.server, user=self.endpoint.user, port=self.endpoint.http_port);
        Some(NativePythonPreamble {
            imports: vec![re, trino],
            from_imports: Vec::new(),
            body: body.to_string(),
        })
    }
    fn get_call(&self) -> AST {
            AST::Call(Call::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("execute_trino_sql".to_string())),
                vec![],
                vec![("query".to_string(), AST::Formatted(Formatted::new_wrapped(
                    self.sql.clone(),
                    self.kwargs.clone(),
                )))].into_iter().collect(),
            ))
    }
}
impl AirflowTaskBase for PrestoPythonTask {
    fn get_dependencies(&self) -> Option<AST> {
        self.dependencies.clone()
    }
}
