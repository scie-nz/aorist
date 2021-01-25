use crate::constraint::{
    AoristStatement, ArgType,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

pub trait ETLSingleton {
    fn get_preamble(&self) -> Option<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> ArgType;
    fn new(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self;
    fn get_statements(&self) -> Vec<AoristStatement>;
    fn compute_task_call(dialect: Option<Dialect>, call: Option<String>) -> ArgType;
}
