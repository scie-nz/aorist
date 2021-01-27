use crate::constraint::{AoristStatement, ArgType, Import};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Location, Statement};

pub trait ETLSingleton {
    fn get_preamble(&self) -> Vec<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> ArgType;
    fn new(
        task_id: ArgType,
        // TODO: change this to optional dict
        task_val: ArgType,
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self;
    fn compute_task_call(&self) -> ArgType;
    fn compute_task_args(&self) -> Vec<ArgType>;
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType>;
    fn get_statements(&self) -> Vec<AoristStatement>;
    fn get_type() -> String;
    fn get_imports(&self) -> Vec<Import>;
    fn build_flow(statements: Vec<Statement>, _location: Location) -> Vec<Statement>;
    fn get_flow_imports() -> Vec<Import>;
}
