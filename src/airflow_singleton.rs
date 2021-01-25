use crate::constraint::{
    AoristStatement, ArgType, Call, Dict, Formatted, List, SimpleIdentifier, StringLiteral,
};
use crate::etl_singleton::ETLSingleton;
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AirflowSingleton {
    task_val: ArgType,
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
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
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
        Self {
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
        Vec::new()
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, ArgType> {
        let mut kwargs;
        if self.dialect.is_none() {
            kwargs = self.kwargs.clone();
        } else {
            kwargs = LinkedHashMap::new();
            let call_param_name = match self.dialect {
                Some(Dialect::Python(_)) => "python_callable".to_string(),
                Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => "bash_command".to_string(),
                _ => panic!("Dialect not supported"),
            };
            let call_param_value = match self.dialect {
                Some(Dialect::Python(_)) => ArgType::SimpleIdentifier(
                    SimpleIdentifier::new_wrapped(self.command.as_ref().unwrap().clone()),
                ),
                Some(Dialect::Bash(_)) => ArgType::Formatted(Formatted::new_wrapped(
                    ArgType::StringLiteral(StringLiteral::new_wrapped(
                        self.command.as_ref().unwrap().clone(),
                    )),
                    self.kwargs.clone(),
                )),
                Some(Dialect::Presto(_)) => {
                    let mut fmt_args = LinkedHashMap::new();
                    fmt_args.insert(
                        "query".to_string(),
                        ArgType::Formatted(Formatted::new_wrapped(
                            ArgType::StringLiteral(StringLiteral::new_wrapped(
                                self.command.as_ref().unwrap().clone(),
                            )),
                            self.kwargs.clone(),
                        )),
                    );

                    ArgType::Formatted(Formatted::new_wrapped(
                        ArgType::StringLiteral(StringLiteral::new_wrapped(
                            "presto -e \"{query}\"".to_string(),
                        )),
                        fmt_args,
                    ))
                }
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
        }
        kwargs.insert(
            "dag".to_string(),
            ArgType::SimpleIdentifier(SimpleIdentifier::new_wrapped("dag".to_string())),
        );
        if let Some(ref dependencies) = self.dep_list {
            if let ArgType::List(_) = dependencies {
                kwargs.insert("dep_list".to_string(), dependencies.clone());
            } else {
                kwargs.insert(
                    "dep_list".to_string(),
                    ArgType::List(List::new_wrapped(vec![dependencies.clone()])),
                );
            }
        }
        kwargs
    }
    fn compute_task_call(&self) -> ArgType {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(ArgType::SimpleIdentifier(
                SimpleIdentifier::new_wrapped("PythonOperator".to_string()),
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
