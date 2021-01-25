use crate::constraint::{AoristStatement, ArgType, Attribute, Call, SimpleIdentifier};
use crate::etl_singleton::ETLSingleton;
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
}
impl ETLSingleton for PrefectSingleton {
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
        let task_creation = AoristStatement::Assign(self.task_val.clone(), creation_expr);
        let mut stmts = vec![task_creation];
        stmts.push(self.get_flow_node_addition());
        for stmt in self.get_edge_addition_statements() {
            stmts.push(stmt);
        }
        stmts
    }
    fn new(
        task_val: ArgType,
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        let task_call = Self::compute_task_call(dialect.clone(), call);
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
    fn compute_task_call(dialect: Option<Dialect>, call: Option<String>) -> ArgType {
        match dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(call.unwrap()),
            )),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".to_string()),
            )),
            None => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "ConstantTask".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
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
