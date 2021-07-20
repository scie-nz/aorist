use crate::python::PythonImport;
use aorist_ast::{AST, Call, Expression, StringLiteral, Attribute, SimpleIdentifier};
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use crate::python::ast::{PythonTaskBase, PythonStatementsTask};
use linked_hash_map::LinkedHashMap;

define_task_node!(
    RPythonTask,
    |task: &RPythonTask| vec![task.call.clone()],
    |task: &RPythonTask| { task.get_native_python_statements() },
    |_task: &RPythonTask| {
        vec![
            PythonImport::PythonModuleImport("rpy2".to_string(), None),
            PythonImport::PythonModuleImport("rpy2.objects".to_string(), Some("robjects".to_string())),
        ]
    },
    PythonImport,
    task_val: AST,
    call: AST,
    args: Vec<AST>,
    kwargs: LinkedHashMap<String, AST>,
    dep_list: Option<AST>,
    preamble: Option<String>,
);
impl PythonStatementsTask for RPythonTask {
    fn python_statements(&self) -> Vec<AST> {
        let mut statements = Vec::new();
        // TODO: push this to a preamble
        let attr = AST::Attribute(Attribute::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("rpy2".into())),
            "r".into(),
            false,
        ));
        if let Some(ref p) = self.preamble {
            let literal = AST::StringLiteral(StringLiteral::new_wrapped(
                format!("\n{}\n", p).to_string(),
                false,
            ));
            let call = AST::Call(Call::new_wrapped(
                attr,
                vec![literal],
                LinkedHashMap::new(),
            ));
            let expr = AST::Expression(Expression::new_wrapped(call));
            statements.push(expr);
        }
        let expr = AST::Expression(Expression::new_wrapped(
            AST::Call(Call::new_wrapped(
                AST::Call(Call::new_wrapped(
                    AST::Attribute(Attribute::new_wrapped(
                        AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("robjects".to_string())),
                        "r".to_string(),
                        false,
                    )),
                    vec![self.call.clone()],
                    LinkedHashMap::new(),
                )),
                self.args.clone(),
                self.kwargs.clone(),
            ))
        ));
        statements.push(expr);
        statements
     }
}
impl PythonTaskBase for RPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
      
