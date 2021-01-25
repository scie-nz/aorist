use crate::constraint::{AoristStatement, ArgType, Call};
use crate::etl_singleton::ETLSingleton;
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AirflowSingleton {
    task_val: ArgType,
    task_call: ArgType,
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
    dep_list: Option<ArgType>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
}
impl ETLSingleton for AirflowSingleton {
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> ArgType {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = ArgType::Call(Call::new_wrapped(
            self.task_call.clone(),
            self.args.clone(),
            self.kwargs.clone(),
        ));
        vec![AoristStatement::Assign(
            self.task_val.clone(),
            creation_expr,
        )]
    }
    fn new(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_val,
            task_call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
        }
    }
}
