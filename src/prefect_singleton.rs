use crate::constraint::{
    AoristStatement, ArgType, Attribute, Call, SimpleIdentifier,
};
use crate::etl_singleton::{ETLSingleton, TDeconstructedSingleton};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PrefectSingleton {
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
impl ETLSingleton for PrefectSingleton {
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
        let task_creation = AoristStatement::Assign(self.task_val.clone(), creation_expr);
        let mut stmts = vec![task_creation];
        stmts.push(self.get_flow_node_addition());
        for stmt in self.get_edge_addition_statements() {
            stmts.push(stmt);
        }
        stmts
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
impl PrefectSingleton {
    fn get_flow_add_edge_statement(&self, dep: ArgType) -> AoristStatement {
        let function = ArgType::Attribute(Attribute::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("flow".to_string())),
            "add_edge".to_string(),
        ));
        let add_expr = ArgType::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val(), dep],
            LinkedHashMap::new(),
        ));
        AoristStatement::Expression(add_expr)
    }
    pub fn get_edge_addition_statements(&self) -> Vec<AoristStatement> {
        match self.dep_list {
            None => Vec::new(),
            Some(ArgType::List(_)) => {
                let target =
                    ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("dep".to_string()));
                let for_stmt = AoristStatement::For(
                    target.clone(),
                    self.dep_list.as_ref().unwrap().clone(),
                    vec![self.get_flow_add_edge_statement(target.clone())],
                );
                vec![for_stmt]
            }
            _ => {
                let dep = self.dep_list.clone();
                let add_stmt = self.get_flow_add_edge_statement(dep.unwrap());
                vec![add_stmt]
            }
        }
    }
    pub fn get_flow_node_addition(&self) -> AoristStatement {
        let function = ArgType::Attribute(Attribute::new_wrapped(
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("flow".to_string())),
            "add_node".to_string(),
        ));
        let add_expr = ArgType::Call(Call::new_wrapped(
            function,
            vec![self.get_task_val()],
            LinkedHashMap::new(),
        ));
        AoristStatement::Expression(add_expr)
    }
}
