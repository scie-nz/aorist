use aorist_ast::{Assignment, AST, StringLiteral, FunctionDef};
use crate::python::ast::PythonTaskBase;
use crate::python::NativePythonPreamble;
use linked_hash_map::LinkedHashMap;

pub trait PythonFunctionCallTask: PythonTaskBase {
    fn python_statements(&self) -> Vec<AST>;
    fn get_preamble(&self) -> Option<NativePythonPreamble> {
        None
    }

    fn statements_as_function(
        &self,
        name: String,
        args: Vec<AST>,
        kwargs: LinkedHashMap<String, AST>,
    ) -> FunctionDef {
        FunctionDef::new(
            name,
            args,
            kwargs,
            self.python_statements(),
        )
    }
    fn get_native_python_statements(&self) -> Vec<AST> {
        let mut statements: Vec<AST> = Vec::new();

        let mut it = self.python_statements().into_iter();

        let mut maybe_statement = it.next();
        let mut task_val_assigned = false;
        while let Some(statement) = maybe_statement {
            maybe_statement = it.next();
            statements.push(match statement {
                AST::Assignment(_) => statement.clone(),
                AST::Expression(ref expr) => match maybe_statement {
                    Some(_) => statement,
                    None => {
                        task_val_assigned = true;
                        AST::Assignment(Assignment::new_wrapped(
                            self.get_task_val(),
                            expr.read().unwrap().inner().clone(),
                        ))
                    }
                },
                _ => panic!(
                    "AST node of type {} found in NativePythonTask body",
                    statement.name()
                ),
            });
        }
        if !task_val_assigned {
            statements.push(AST::Assignment(Assignment::new_wrapped(
                self.get_task_val().clone(),
                AST::StringLiteral(StringLiteral::new_wrapped("Done".to_string(), false)),
            )));
        }
        statements
    }
}
