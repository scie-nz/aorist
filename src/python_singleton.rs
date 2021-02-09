use crate::etl_singleton::{ETLSingleton, ETLDAG};
use crate::python::{
    AoristStatement, Attribute, Call, Formatted, Import, SimpleIdentifier, StringLiteral, Tuple,
    AST,
};
use aorist_primitives::Dialect;
use linked_hash_map::LinkedHashMap;
use pyo3::prelude::*;
use pyo3::types::PyModule;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PythonSingleton {
    task_id: AST,
    task_val: AST,
    command: Option<String>,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
    dialect: Option<Dialect>,
}
impl ETLSingleton for PythonSingleton {
    fn get_imports(&self) -> Vec<Import> {
        match self.dialect {
            Some(Dialect::Python(_)) | None => vec![],
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                vec![Import::ModuleImport("subprocess".to_string())]
            }
        }
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
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
    fn get_statements(&self) -> Vec<AoristStatement> {
        let creation_expr = AST::Call(Call::new_wrapped(
            self.compute_task_call(),
            self.compute_task_args(),
            self.compute_task_kwargs(),
        ));
        match self.dialect {
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) | Some(Dialect::R(_)) => {
                let process =
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("process".to_string()));
                let task_creation = AoristStatement::Assign(process.clone(), creation_expr);
                let task_assign = AoristStatement::Assign(
                    AST::Tuple(Tuple::new_wrapped(
                        vec![
                            self.get_task_val().as_wrapped_assignment_target(),
                            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                                "error".to_string(),
                            )),
                        ],
                        true,
                    )),
                    AST::Call(Call::new_wrapped(
                        AST::Attribute(Attribute::new_wrapped(
                            process,
                            "communicate".to_string(),
                            false,
                        )),
                        Vec::new(),
                        LinkedHashMap::new(),
                    )),
                );
                vec![task_creation, task_assign]
            }
            _ => vec![AoristStatement::Expression(creation_expr)],
        }
    }
    fn new(
        task_id: AST,
        task_val: AST,
        call: Option<String>,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
        dep_list: Option<AST>,
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
    fn compute_task_args(&self) -> Vec<AST> {
        match self.dialect {
            Some(Dialect::Python(_)) => self.args.clone(),
            Some(Dialect::Bash(_)) => vec![AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    self.command.as_ref().unwrap().clone(),
                )),
                self.kwargs.clone(),
            ))],
            Some(Dialect::Presto(_)) => vec![AST::Formatted(Formatted::new_wrapped(
                AST::StringLiteral(StringLiteral::new_wrapped(
                    format!("presto -e '{}'", self.command.as_ref().unwrap()).to_string(),
                )),
                self.kwargs.clone(),
            ))],
            None => vec![self.task_id.clone()],
            _ => panic!("Dialect not supported"),
        }
    }
    fn compute_task_kwargs(&self) -> LinkedHashMap<String, AST> {
        match self.dialect {
            Some(Dialect::Python(_)) => self.kwargs.clone(),
            None => LinkedHashMap::new(),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => {
                let mut kwargs = LinkedHashMap::new();
                kwargs.insert(
                    "stdout".to_string(),
                    AST::Attribute(Attribute::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                            "subprocess".to_string(),
                        )),
                        "PIPE".to_string(),
                        false,
                    )),
                );
                kwargs
            }
            _ => panic!("Dialect not supported"),
        }
    }
    fn compute_task_call(&self) -> AST {
        match self.dialect {
            Some(Dialect::Python(_)) => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                self.command.as_ref().unwrap().clone(),
            ))),
            Some(Dialect::Bash(_)) | Some(Dialect::Presto(_)) => {
                Ok(AST::Attribute(Attribute::new_wrapped(
                    AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("subprocess".to_string())),
                    "popen".to_string(),
                    false,
                )))
            }
            None => Ok(AST::SimpleIdentifier(SimpleIdentifier::new_wrapped(
                "print".to_string(),
            ))),
            _ => Err("Dialect not supported".to_string()),
        }
        .unwrap()
    }
    fn get_type() -> String {
        "python".to_string()
    }
}
pub struct PythonDAG {}
impl ETLDAG for PythonDAG {
    type T = PythonSingleton;
    fn new() -> Self {
        Self {}
    }
    fn get_flow_imports(&self) -> Vec<Import> {
        Vec::new()
    }
    /// Takes a set of statements and mutates them so as make a valid ETL flow
    fn build_flow<'a>(
        &self,
        _py: Python<'a>,
        statements: Vec<&'a PyAny>,
        _ast_module: &'a PyModule,
    ) -> Vec<&'a PyAny> {
        statements
    }
}
