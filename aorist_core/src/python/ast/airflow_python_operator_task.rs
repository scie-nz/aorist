use crate::python::PythonImport;
use aorist_ast::{Assignment, Attribute, Call, SimpleIdentifier, AST, Expression, Dict};
use linked_hash_map::LinkedHashMap;
use crate::python::ast::PythonTaskBase;
use crate::python::ast::AirflowTaskBase;

pub trait AirflowPythonOperatorTask: PythonTaskBase + AirflowTaskBase {
    fn compute_task_call(&self) -> AST {
         AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".to_string()))
    }
    fn get_call_param_value(&self) -> AST;
    fn get_python_bash_operator_imports(&self) -> Vec<PythonImport> {
        vec![PythonImport::PythonFromImport(
            "airflow.operators.python_operator".to_string(),
            "PythonOperator".to_string(),
            None,
        )]
    }
    fn get_callable_kwargs(&self) -> LinkedHashMap<String, AST>;
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        let mut kwargs = LinkedHashMap::new();
        let call_param_value = self.get_call_param_value();
        kwargs.insert("python_callable".to_string(), call_param_value);
        let callable_kwargs = self.get_callable_kwargs();
        if callable_kwargs.len() > 0 {
            kwargs.insert(
                "op_kwargs".to_string(),
                AST::Dict(Dict::new_wrapped(callable_kwargs)),
            );
        }
        kwargs
    }
    fn get_operator_statements(&self) -> Vec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            vec![],
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AST::Assignment(Assignment::new_wrapped(
            self.get_task_val(),
            creation_expr,
        ))];
        if let Some(dependencies) = self.get_dependencies() {
            statements.push(AST::Expression(Expression::new_wrapped(AST::Call(
                Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".to_string(),
                        false,
                    )),
                    vec![dependencies],
                    LinkedHashMap::new(),
                ),
            ))));
        }
        statements
    }
}
