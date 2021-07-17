use crate::python::PythonImport;
use aorist_ast::{Assignment, Call, Expression, SimpleIdentifier, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub trait PythonStatementsTask {
    fn python_statements(&self) -> Vec<AST>;
    fn get_task_val(&self) -> AST;
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

define_task_node!(
    ConstantPythonTask,
    |task: &ConstantPythonTask| vec![task.name.clone()],
    |task: &ConstantPythonTask| { task.get_native_python_statements() },
    |_task: &ConstantPythonTask| { vec![] },
    PythonImport,
    name: AST,
    task_val: AST,
);
impl PythonStatementsTask for ConstantPythonTask {
   fn get_task_val(&self) -> AST {
      self.task_val.clone()
   }
   fn python_statements(&self) -> Vec<AST> {
        let call = AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("print".to_string())),
            vec![self.name.clone()],
            LinkedHashMap::new(),
        ));

        vec![
            call,
            self.name.clone(),
        ]
   }
}
