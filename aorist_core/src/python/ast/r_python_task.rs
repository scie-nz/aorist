use crate::python::ast::{PythonFunctionCallTask, PythonTaskBase};
use crate::python::NativePythonPreamble;
use crate::python::PythonImport;
use aorist_ast::{Call, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::define_task_node;
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

define_task_node!(
    RPythonTask,
    |task: &RPythonTask| vec![task.call.clone()],
    |task: &RPythonTask| { task.get_native_python_statements() },
    |_task: &RPythonTask| {
        vec![
            PythonImport::PythonModuleImport("rpy2".to_string(), None),
            PythonImport::PythonModuleImport(
                "rpy2.robjects".to_string(),
                Some("robjects".to_string()),
            ),
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
        let rpy2o = PythonImport::PythonModuleImport(
            "rpy2.robjects".to_string(),
            Some("robjects".to_string()),
        );
        let body = "
def execute_r(call, preamble=None, **kwargs):
    if preamble is not None:
        robjects.r(preamble)
    return robjects.r(
        \"%s(%s)\"
        % (call, ', '.join([
            ('%s = \"%s\"' % (k, v))
            if isinstance(v, str) 
            else (('%s = %s' % (k, str(v)))
                  if isinstance(v, int) or isinstance(v, float)
                  else ('%s = c(%s)' % (k, ', '.join(v)))
                 )
            for k, v in kwargs.items()
        ]))
    )

";
        Some(NativePythonPreamble {
            imports: vec![rpy2],
            from_imports: vec![rpy2o],
            body: body.to_string(),
        })
    }
    fn get_call(&self) -> AST {
        let mut inner_kwargs = LinkedHashMap::new();
        inner_kwargs.insert("call".to_string(), self.call.clone());
        if let Some(ref p) = self.preamble {
            inner_kwargs.insert(
                "preamble".to_string(),
                AST::StringLiteral(StringLiteral::new_wrapped(
                    format!("\n{}\n", p).to_string(),
                    false,
                )),
            );
        }
        for (k, v) in self.kwargs.clone() {
            inner_kwargs.insert(k, v);
        }
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("execute_r".to_string())),
            vec![],
            inner_kwargs,
        ))
    }
}
impl PythonTaskBase for RPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
