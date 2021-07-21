use crate::python::PythonImport;
use aorist_ast::{AST, Call, StringLiteral, SimpleIdentifier, Formatted};
use aorist_primitives::define_task_node;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use crate::python::ast::{PythonTaskBase, PythonFunctionCallTask};
use linked_hash_map::LinkedHashMap;
use crate::python::NativePythonPreamble;

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
impl PythonFunctionCallTask for RPythonTask {
    
    fn get_preamble(&self) -> Option<NativePythonPreamble> {
        let rpy2 = PythonImport::PythonModuleImport("rpy2".to_string(), None);
        let rpy2o = PythonImport::PythonModuleImport("rpy2.objects".to_string(), Some("robjects".to_string()));
        let body = "
def execute_r(call, preamble=None):
    if preamble is not None:
        rpy2.r(preamble)
    return rpy2.r(call)
";
        Some(NativePythonPreamble {
            imports: vec![rpy2],
            from_imports: vec![rpy2o],
            body: body.to_string(),
        })
    }
    fn get_call(&self) -> AST {
        let mut args = vec![];
        let expr = AST::Formatted(Formatted::new_wrapped(
            self.call.clone(),
            self.kwargs.clone(),
        ));
        args.push(expr);
        if let Some(ref p) = self.preamble {
            let literal = AST::StringLiteral(StringLiteral::new_wrapped(
                format!("\n{}\n", p).to_string(),
                false,
            ));
            args.push(literal);
        }
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("execute_r".to_string())),
            args,
            LinkedHashMap::new(),
        ))
     }
}
impl PythonTaskBase for RPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
      
