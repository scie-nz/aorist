use crate::code::{Import, Preamble};
use crate::concept::AoristRef;
use crate::dialect::Dialect;
use crate::endpoints::EndpointConfig;
use aorist_ast::AST;

use linked_hash_map::LinkedHashMap;

/// Encapsulates the abstract bits necessary for the creation of an ETL Flow
pub trait ETLFlow {
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
        endpoints: AoristRef<EndpointConfig>,
    ) -> Self;
    fn get_statements(&self) -> Vec<AST>;
    fn get_type() -> String;
    fn get_imports(&self) -> Vec<Self::ImportType>;
}
