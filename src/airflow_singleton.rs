use crate::constraint::{
    AoristStatement, ArgType, Call, Dict, Formatted, SimpleIdentifier, StringLiteral,
};
use crate::etl_singleton::ETLSingleton;
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AirflowSingleton {
    task_val: ArgType,
    task_call: ArgType,
    command: Option<String>,
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
        call: Option<String>,
        args: Vec<ArgType>,
        kwargs: LinkedHashMap<String, ArgType>,
        dep_list: Option<ArgType>,
        preamble: Option<String>,
        dialect: Option<Dialect>,
    ) -> Self {
        let task_call = Self::compute_task_call(dialect.clone(), call.clone());
        Self {
            task_val,
            task_call,
            command: call,
            args,
            kwargs,
            dep_list,
            preamble,
            dialect,
        }
    }
    fn compute_task_args(&self) -> Vec<ArgType> {
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType> {
        if self.dialect.is_none() {
            return self.kwargs.clone();
        }
        let mut kwargs = LinkedHashMap::new();
        let call_param_name = match self.dialect {
            Some(Dialect::Python(_)) => "python_callable".to_string(),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "bash_command".to_string(),
            _ => panic!("Dialect not supported"),
        };
        let call_param_value = match self.dialect {
            Some(Dialect::Python(_)) => ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.command.as_ref().unwrap().clone(),
            )),
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
        if let Some(Dialect::Python(_)) = self.dialect {
            if self.kwargs.len() > 0 {
                kwargs.insert(
                    "op_kwargs".to_string(),
                    ArgType::Dict(Dict::new_wrapped(self.kwargs.clone())),
                );
            }
        }
        kwargs.insert(
            "dag".to_string(),
            ArgType::StringLiteral(StringLiteral::new_wrapped("dag".to_string())),
        );
        kwargs
    }
    fn compute_task_call(dialect: Option<Dialect>, call: Option<String>) -> ArgType {
        match dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped(call.unwrap()),
            )),
            Some(Dialect::Bash(_)) => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "BashOperator".to_string(),
            ))),
            Some(Dialect::Presto(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("BashOperator".to_string()),
            )),
            None => Ok(ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "DummyOperator".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
}
