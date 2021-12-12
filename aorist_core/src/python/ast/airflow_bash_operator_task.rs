use crate::python::ast::AirflowTaskBase;
use crate::python::ast::PythonTaskBase;
use crate::python::PythonImport;
use abi_stable::std_types::ROption;
use aorist_ast::{Assignment, Attribute, Call, Expression, SimpleIdentifier, AST};
use aorist_primitives::AOption;
use aorist_primitives::{AString, AVec};
use linked_hash_map::LinkedHashMap;

pub trait AirflowBashOperatorTask: PythonTaskBase + AirflowTaskBase {
    fn compute_task_call(&self) -> AST {
        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("BashOperator".into()))
    }
    fn get_call_param_value(&self) -> AST;
    fn get_python_bash_operator_imports(&self) -> AVec<PythonImport> {
        vec![PythonImport::PythonFromImport(
            "airflow.operators.bash_operator".into(),
            "BashOperator".into(),
            AOption(ROption::RNone),
        )]
        .into_iter()
        .collect()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        let mut kwargs = LinkedHashMap::new();
        let call_param_value = self.get_call_param_value();
        kwargs.insert("bash_command".into(), call_param_value);
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
