use crate::constraint::{
    AoristStatement, ArgType, List, StringLiteral, Subscript,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Location, Suite};
pub type TDeconstructedSingleton = (
    ArgType,
    ArgType,
    ArgType,
    Vec<ArgType>,
    LinkedHashMap<String, ArgType>,
    Option<ArgType>,
    Option<String>,
    Option<Dialect>,
);

pub trait ETLSingleton {
    fn get_preamble(&self) -> Option<String>;
    fn get_dialect(&self) -> Option<Dialect>;
    fn get_task_val(&self) -> ArgType;
    fn get_assign_statements(&self) -> Vec<AoristStatement>;
    fn deconstruct(&self) -> Option<TDeconstructedSingleton>;
    fn new(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        referenced_dict: Option<(ArgType, ArgType)>,
    ) -> Self;
    fn new_referencing_dict(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwarg_keys: &Vec<String>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        params: (ArgType, ArgType),
    ) -> Self
    where
        Self: Sized,
    {
        // HACK
        let kwargs = kwarg_keys
            .iter()
            .map(|x| {
                (
                    x.clone(),
                    ArgType::Subscript(Subscript::new_wrapped(
                        params.1.clone(),
                        ArgType::StringLiteral(StringLiteral::new_wrapped(x.to_string())),
                    )),
                )
            })
            .collect::<LinkedHashMap<_, _>>();
        let mut future_list = ArgType::List(List::new_wrapped(vec![]));
        future_list.set_owner(ArgType::Subscript(Subscript::new_wrapped(
            params.1.clone(),
            ArgType::StringLiteral(StringLiteral::new_wrapped("dep_list".to_string())),
        )));
        Self::new(
            task_val,
            task_call,
            args,
            kwargs,
            Some(future_list),
            preamble,
            dialect,
            Some(params),
        )
    }
    fn as_suite(&self, location: Location) -> Suite {
        self.get_assign_statements()
            .into_iter()
            .map(|x| x.statement(location))
            .collect::<Vec<_>>()
    }
}
