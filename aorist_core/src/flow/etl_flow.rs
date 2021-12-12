use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::code::{Import, Preamble};
use crate::dialect::Dialect;
use aorist_ast::AST;
use aorist_primitives::{AString, AVec, AoristUniverse};
use linked_hash_map::LinkedHashMap;
use std::error::Error;

/// Encapsulates the abstract bits necessary for the creation of an ETL Flow
pub trait ETLFlow<U>
where
    U: AoristUniverse,
{
    type ImportType: Import;
    type ErrorType: Error;
    type PreambleType: Preamble<ImportType = Self::ImportType>;

    fn get_preamble(&self) -> Result<AVec<Self::PreambleType>, Self::ErrorType>;
    fn get_dialect(&self) -> AOption<Dialect>;
    fn get_task_val(&self) -> AST;
    fn new(
        task_id: AST,
        // TODO: change this to optional dict
        task_val: AST,
        call: AOption<AString>,
        args: AVec<AST>,
        kwargs: LinkedHashMap<AString, AST>,
        dep_list: AOption<AST>,
        preamble: AOption<AString>,
        dialect: AOption<Dialect>,
        endpoints: U::TEndpoints,
    ) -> Self;
    fn get_statements(&self) -> AVec<AST>;
    fn get_type() -> String;
    fn get_imports(&self) -> AVec<Self::ImportType>;
}
