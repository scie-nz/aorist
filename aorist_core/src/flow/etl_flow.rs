use crate::code::{Import, Preamble};
use crate::dialect::Dialect;
use aorist_ast::AST;
use aorist_primitives::AoristUniverse;
use linked_hash_map::LinkedHashMap;

/// Encapsulates the abstract bits necessary for the creation of an ETL Flow
pub trait ETLFlow<U>
where
    U: AoristUniverse,
{
    type ImportType: Import;
    type PreambleType: Preamble<ImportType = Self::ImportType>;

    fn get_preamble(&self) -> Vec<Self::PreambleType>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> AST;
    fn new(
        task_id: AST,
        // TODO: change this to optional dict
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        endpoints: U::TEndpoints,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_type() -> String;
    fn get_imports(&self) -> Vec<Self::ImportType>;
}
