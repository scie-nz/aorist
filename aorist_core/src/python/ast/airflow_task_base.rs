use aorist_ast::AST;
use aorist_primitives::AOption;

pub trait AirflowTaskBase {
    fn get_dependencies(&self) -> AOption<AST>;
}
