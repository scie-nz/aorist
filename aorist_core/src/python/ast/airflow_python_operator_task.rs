use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::python::ast::AirflowTaskBase;
use crate::python::ast::PythonTaskBase;
use crate::python::PythonImport;
use aorist_ast::{Assignment, Attribute, Call, Dict, Expression, SimpleIdentifier, AST};
use aorist_primitives::{AString, AVec};
use linked_hash_map::LinkedHashMap;

pub trait AirflowPythonOperatorTask: PythonTaskBase + AirflowTaskBase {
    fn compute_task_call(&self) -> AST {
        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("PythonOperator".into()))
    }
    fn get_call_param_value(&self) -> AST;
    fn get_python_operator_imports(&self) -> AVec<PythonImport> {
        vec![PythonImport::PythonFromImport(
            "airflow.operators.python_operator".into(),
            "PythonOperator".into(),
            AOption(ROption::RNone),
        )]
        .into_iter()
        .collect()
    }
    fn get_callable_kwargs(&self) -> LinkedHashMap<AString, AST>;
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        let mut kwargs = LinkedHashMap::new();
        let call_param_value = self.get_call_param_value();
        kwargs.insert("python_callable".into(), call_param_value);
        let callable_kwargs = self.get_callable_kwargs();
        if callable_kwargs.len() > 0 {
            kwargs.insert(
                "op_kwargs".into(),
                AST::Dict(Dict::new_wrapped(callable_kwargs)),
            );
        }
        kwargs
    }
    fn get_operator_statements(&self) -> AVec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            vec![].into_iter().collect(),
            self.compute_task_kwargs(),
        ));
        let mut statements = vec![AST::Assignment(Assignment::new_wrapped(
            self.get_task_val(),
            creation_expr,
        ))];
        if let AOption(ROption::RSome(dependencies)) = self.get_dependencies() {
            statements.push(AST::Expression(Expression::new_wrapped(AST::Call(
                Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        self.get_task_val(),
                        "set_upstream".into(),
                        false,
                    )),
                    vec![dependencies].into_iter().collect(),
                    LinkedHashMap::new(),
                ),
            ))));
        }
        statements.into_iter().collect()
    }
}
