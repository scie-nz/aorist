#![allow(dead_code)]
use crate::python::ast::{
    Assignment, Attribute, BooleanLiteral, Call, Formatted, Import, SimpleIdentifier, Tuple, AST,
};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub trait TAssignmentTarget
where
    Self: Sized,
{
    fn as_assignment_target(&self) -> Self;
    fn as_wrapped_assignment_target(&self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self.as_assignment_target()))
    }
}

define_task_node!(
    BashPythonTask,
    |task: &BashPythonTask| vec![task.command.clone()]
        .into_iter()
        .chain(task.kwargs.values().map(|x| x.clone()))
        .collect(),
    |task: &BashPythonTask| { task.get_subprocess_statements() },
    |_task: &BashPythonTask| { vec![Import::ModuleImport("subprocess".to_string())] },
    task_val: AST,
    command: AST,
    kwargs: LinkedHashMap<String, AST>,
);

trait PythonSubprocessTask {
    fn compute_task_call(&self) -> AST {
        AST::Attribute(Attribute::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".to_string())),
            "Popen".to_string(),
            false,
        ))
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        let mut kwargs = LinkedHashMap::new();
        kwargs.insert(
            "stdout".to_string(),
            AST::Attribute(Attribute::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".to_string())),
                "PIPE".to_string(),
                false,
            )),
        );
        kwargs.insert(
            "shell".to_string(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(true)),
        );
        kwargs
    }
    fn get_command(&self) -> AST;
    fn get_task_val(&self) -> AST;
    fn get_subprocess_statements(&self) -> Vec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            vec![self.get_command()],
            self.compute_task_kwargs(),
        ));
        let process = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("process".to_string()));
        let task_creation =
            AST::Assignment(Assignment::new_wrapped(process.clone(), creation_expr));
        let task_assign = AST::Assignment(Assignment::new_wrapped(
            AST::Tuple(Tuple::new_wrapped(
                vec![
                    self.get_task_val().as_wrapped_assignment_target(),
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("error".to_string())),
                ],
                true,
            )),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(
                    process,
                    "communicate".to_string(),
                    false,
                )),
                Vec::new(),
                LinkedHashMap::new(),
            )),
        ));
        vec![task_creation, task_assign]
    }
}
impl PythonSubprocessTask for BashPythonTask {
    fn get_command(&self) -> AST {
        AST::Formatted(Formatted::new_wrapped(
            self.command.clone(),
            self.kwargs.clone(),
        ))
    }
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
