use crate::python::ast::PythonTaskBase;
use crate::python::NativePythonPreamble;
use abi_stable::std_types::ROption;
use aorist_ast::{Assignment, AST};
use aorist_util::AOption;
use aorist_util::AVec;

pub trait PythonFunctionCallTask: PythonTaskBase {
    fn get_call(&self) -> AST;
    fn get_preamble(&self) -> AOption<NativePythonPreamble> {
        AOption(ROption::RNone)
    }

    fn get_native_python_statements(&self) -> AVec<AST> {
        vec![AST::Assignment(Assignment::new_wrapped(
            self.get_task_val(),
            self.get_call(),
        ))]
        .into_iter()
        .collect()
    }
}
