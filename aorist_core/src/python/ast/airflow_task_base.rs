use aorist_ast::AST;

pub trait AirflowTaskBase {
    fn get_dependencies(&self) -> Option<AST>;
}
