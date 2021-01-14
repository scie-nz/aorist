use crate::constraint::{
    AoristStatement, ArgType, List, StringLiteral, Subscript, Call, Attribute,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Location, Suite};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

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
    fn get_referenced_dict(&self) -> &Option<(ArgType, ArgType)>;
    fn get_assign_statements(&self) -> Vec<AoristStatement> {
        if let Some((tpl, dict)) = self.get_referenced_dict() {
            let dict_descendants = dict.get_descendants();
            let mut values_to_assign: HashMap<ArgType, ArgType> = HashMap::new();
            for desc in dict_descendants {
                if let ArgType::StringLiteral(ref literal) = desc {
                    let maybe_owner = desc.get_owner();
                    if let Some(owner) = maybe_owner {
                        if let ArgType::SimpleIdentifier { .. } = owner {
                            // this does not have an owner so it
                            // will render correctly
                            let desc_deep_clone = ArgType::StringLiteral(
                                StringLiteral::new_wrapped(literal.read().unwrap().value()),
                            );
                            if let Some(val) = values_to_assign.get(&owner) {
                                assert!(*val == desc_deep_clone);
                            } else {
                                values_to_assign.insert(owner, desc_deep_clone);
                            }
                        }
                    }
                }
            }
            let mut assign_statements = values_to_assign
                .into_iter()
                .map(|(k, v)| AoristStatement::Assign(k, v))
                .collect::<Vec<_>>();

            let statements = self.get_statements();
            let items_call = ArgType::Call(Call::new_wrapped(
                ArgType::Attribute(Attribute::new_wrapped(dict.clone(), "items".to_string())),
                Vec::new(),
                LinkedHashMap::new(),
            ));
            let for_loop = AoristStatement::For(tpl.clone(), items_call, statements.clone());
            let dict_name = dict.get_ultimate_owner().unwrap();
            // HACK ALERT
            let assign;
            if let ArgType::Dict(x) = dict.clone() {
                let mut dict_raw = ArgType::Dict(Arc::new(RwLock::new(x.read().unwrap().clone())));
                dict_raw.remove_owner();
                assign = AoristStatement::Assign(dict_name, dict_raw);
            } else {
                panic!("dict should be a Dict");
            }
            assign_statements.push(assign);
            assign_statements.push(for_loop);
            return assign_statements;
        }
        // TODO: assignment statements for any other args go here
        self.get_statements()
    }
    fn get_statements(&self) -> Vec<AoristStatement>;
}
