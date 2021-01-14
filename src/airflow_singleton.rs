use crate::constraint::{
    AoristStatement, ArgType, Call,
};
use crate::etl_singleton::{ETLSingleton, TDeconstructedSingleton};
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
    /// parameter / dep_list dictionary, from where the Singleton's dep_list
    /// and keyword arg values are drawn (this is only used for for_loop
    /// compression). First value is the alias in the for loop, second is the
    /// actual dict.
    referenced_dict: Option<(ArgType, ArgType)>,
}
impl ETLSingleton for AirflowSingleton {
    fn get_preamble(&self) -> Option<String> {
        self.preamble.clone()
    }
    fn get_referenced_dict(&self) -> &Option<(ArgType, ArgType)> {
        &self.referenced_dict
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
        vec![AoristStatement::Assign(self.task_val.clone(), creation_expr)]
    }
    fn deconstruct(&self) -> Option<TDeconstructedSingleton> {
        if let ArgType::Subscript(ref subscript) = self.task_val {
            let guard = subscript.read().unwrap();
            return Some((
                guard.a().clone(),
                guard.b().clone(),
                self.task_call.clone(),
                self.args.clone(),
                self.kwargs.clone(),
                self.dep_list.clone(),
                self.preamble.clone(),
                self.dialect.clone(),
            ));
        }
        None
    }
    fn new(
        task_val: ArgType,
        task_call: ArgType,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
        referenced_dict: Option<(ArgType, ArgType)>,
    ) -> Self {
        Self {
            task_val,
            task_call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
            referenced_dict,
        }
    }
}
