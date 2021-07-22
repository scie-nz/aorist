use aorist_ast::AST;

pub trait PythonTaskBase {
    fn get_task_val(&self) -> AST;
}
