use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use aorist_ast::AST;

pub trait AirflowTaskBase {
    fn get_dependencies(&self) -> AOption<AST>;
}
