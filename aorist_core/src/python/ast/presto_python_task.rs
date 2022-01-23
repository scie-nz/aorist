use abi_stable::std_types::ROption;
use aorist_util::AOption;

use crate::python::ast::AirflowTaskBase;
use crate::python::ast::{PythonFunctionCallTask, PythonTaskBase};
use crate::python::NativePythonPreamble;
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use aorist_ast::{Call, Formatted, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use aorist_primitives::PrestoConfig;
use aorist_util::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;

define_task_node!(
    PrestoPythonTask,
    |task: &PrestoPythonTask| vec![task.sql.clone()].into_iter().collect(),
    |task: &PrestoPythonTask| { task.get_native_python_statements() },
    |_task: &PrestoPythonTask| {
        vec![
            PythonImport::PythonModuleImport("subprocess".into(), AOption(ROption::RNone)),
            PythonImport::PythonModuleImport("trino".into(), AOption(ROption::RNone)),
            PythonImport::PythonModuleImport("re".into(), AOption(ROption::RNone)),
        ]
        .into_iter()
        .collect()
    },
    PythonImport,
    sql: AST,
    kwargs: LinkedHashMap<AString, AST>,
    task_val: AST,
    endpoint: PrestoConfig,
    dependencies: AOption<AST>,
);

impl PythonTaskBase for PrestoPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
impl PythonFunctionCallTask for PrestoPythonTask {
    fn get_preamble(&self) -> AOption<NativePythonPreamble> {
        let re = PythonImport::PythonModuleImport("re".into(), AOption(ROption::RNone));
        let trino = PythonImport::PythonModuleImport("trino".into(), AOption(ROption::RNone));
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
        AOption(ROption::RSome(NativePythonPreamble {
            imports: vec![re, trino].into_iter().collect(),
            from_imports: AVec::new(),
            body: body.as_str().into(),
        }))
    }
    fn get_call(&self) -> AST {
        let query;
        if let AST::StringLiteral(ref s) = self.sql {
            if s.read().value().as_str() == "{queries}" {
                query = self.kwargs.get(&("queries".into())).unwrap().clone();
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
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("execute_trino_sql".into())),
            vec![].into_iter().collect(),
            vec![("query".into(), query)].into_iter().collect(),
        ))
    }
}
impl AirflowTaskBase for PrestoPythonTask {
    fn get_dependencies(&self) -> AOption<AST> {
        self.dependencies.clone()
    }
}
