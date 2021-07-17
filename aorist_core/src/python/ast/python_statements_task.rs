use aorist_ast::{Assignment, Expression, AST};
use crate::python::ast::PythonTaskBase;

pub trait PythonStatementsTask: PythonTaskBase {
    fn python_statements(&self) -> Vec<AST>;
    fn get_native_python_statements(&self) -> Vec<AST> {
        let mut statements = self.python_statements();
        let l = statements.len();
        
        let last = statements.remove(l - 1);
        let mut out = Vec::new();
        for elem in statements.into_iter() {
            out.push(
                AST::Expression(Expression::new_wrapped(elem))
            );
        }
        let assignment = AST::Assignment(Assignment::new_wrapped(
            self.get_task_val(),
            last
        ));
        out.push(assignment);
        out
    }
}
