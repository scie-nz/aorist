use crate::constraint::{
    AoristStatement, ArgType, Attribute, Call, Formatted, Import, SimpleIdentifier, StringLiteral,
};
use crate::etl_singleton::ETLSingleton;
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use rustpython_parser::ast::{Location, Statement};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PrefectSingleton {
    task_id: ArgType,
    task_val: ArgType,
    command: Option<String>,
    args: Vec<ArgType>,
    kwargs: LinkedHashMap<String, ArgType>,
    dep_list: Option<ArgType>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
}
impl ETLSingleton for PrefectSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) => vec![Import::FromImport(
                "prefect".to_string(),
                "task".to_string(),
            )],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![Import::FromImport(
                    "prefect.tasks.shell".to_string(),
                    "ShellTask".to_string(),
                )]
            }
            None => vec![Import::FromImport(
                "prefect.tasks.core".to_string(),
                "Constant".to_string(),
            )],
        }
    }
    fn build_flow(statements: Vec<Statement>, _location: Location) -> Vec<Statement> {
        statements
    }
    fn get_preamble(&self) -> Vec<String> {
        let preambles = match self.dialect {
            Some(Dialect::Python(_)) => match self.preamble {
                Some(ref p) => vec![p.clone()],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        preambles
    }
    fn get_dialect(&self) -> Option<Dialect> {
        self.dialect.clone()
    }
    fn get_task_val(&self) -> ArgType {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = ArgType::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        let task_creation = AoristStatement::Assign(self.get_task_val(), creation_expr);
        let mut stmts = vec![task_creation];
        stmts.push(self.get_flow_node_addition());
        for stmt in self.get_edge_addition_statements() {
            stmts.push(stmt);
        }
        stmts
    }
    fn new(
        task_id: ArgType,
        task_val: ArgType,
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        Self {
            task_id,
            task_val,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
        }
    }
    fn compute_task_args(&self) -> Vec<ArgType> {
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.args.clone();
        }
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType> {
        if self.dialect.is_none() {
            return self.kwargs.clone();
        }
        if let Some(Dialect::Python(_)) = self.dialect {
            return self.kwargs.clone();
        }
        let mut kwargs = LinkedHashMap::new();
        let call_param_name = match self.dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "command".to_string(),
            _ => panic!("Dialect not supported"),
        };
        let call_param_value = match self.dialect {
            Some(Dialect::Bash(_)) => ArgType::Formatted(Formatted::new_wrapped(
                ArgType::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.kwargs.clone(),
            )),
            Some(Dialect::Presto(_)) => ArgType::Formatted(Formatted::new_wrapped(
                ArgType::StringLiteral(StringLiteral::new_wrapped(
                    format!("presto -e '{}'", self.command.as_ref().unwrap()).to_string(),
                )),
                self.kwargs.clone(),
            )),
            _ => panic!("Dialect not supported"),
        };
        kwargs.insert(call_param_name, call_param_value);
        kwargs
    }
    fn compute_task_call(&self) -> ArgType {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(self.command.as_ref().unwrap().clone()),
            )),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("ShellTask".to_string()),
            )),
            None => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "Constant".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
    fn get_type() -> String {
        "prefect".to_string()
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
