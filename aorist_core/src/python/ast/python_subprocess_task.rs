use crate::python::ast::PythonTaskBase;
use crate::python::PythonImport;
use abi_stable::std_types::ROption;
use aorist_ast::{Assignment, Attribute, BooleanLiteral, Call, SimpleIdentifier, Tuple, AST};
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use linked_hash_map::LinkedHashMap;

pub trait PythonSubprocessTask: PythonTaskBase {
    fn compute_task_call(&self) -> AST {
        AST::Attribute(Attribute::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".into())),
            "Popen".into(),
            false,
        ))
    }
    fn get_python_subprocess_imports(&self) -> AVec<PythonImport> {
        vec![PythonImport::PythonModuleImport(
            "subprocess".into(),
            AOption(ROption::RNone),
        )]
        .into_iter()
        .collect()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<AString, AST> {
        let mut kwargs = LinkedHashMap::new();
        kwargs.insert(
            "stdout".into(),
            AST::Attribute(Attribute::new_wrapped(
                AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".into())),
                "PIPE".into(),
                false,
            )),
        );
        kwargs.insert(
            "shell".into(),
            AST::BooleanLiteral(BooleanLiteral::new_wrapped(true)),
        );
        kwargs
    }
    fn get_command(&self) -> AST;
    fn get_subprocess_statements(&self) -> AVec<AST> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            vec![self.get_command()].into_iter().collect(),
            self.compute_task_kwargs(),
        ));
        let process = AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("process".into()));
        let task_creation =
            AST::Assignment(Assignment::new_wrapped(process.clone(), creation_expr));
        let task_assign = AST::Assignment(Assignment::new_wrapped(
            AST::Tuple(Tuple::new_wrapped(
                vec![
                    self.get_task_val().as_wrapped_assignment_target(),
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("error".into())),
                ]
                .into_iter()
                .collect(),
                true,
            )),
            AST::Call(Call::new_wrapped(
                AST::Attribute(Attribute::new_wrapped(process, "communicate".into(), false)),
                AVec::new(),
                LinkedHashMap::new(),
            )),
        ));
        vec![task_creation, task_assign].into_iter().collect()
    }
}
