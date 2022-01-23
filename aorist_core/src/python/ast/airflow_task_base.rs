use aorist_ast::AST;
use aorist_util::AOption;

pub trait AirflowTaskBase {
    fn get_dependencies(&self) -> AOption<AST>;
}
