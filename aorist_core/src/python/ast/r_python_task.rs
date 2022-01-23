use crate::python::ast::{PythonFunctionCallTask, PythonTaskBase};
use crate::python::NativePythonPreamble;
use crate::python::PythonImport;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use abi_stable::std_types::ROption;
use aorist_ast::{Call, SimpleIdentifier, StringLiteral, AST};
use aorist_primitives::define_task_node;
use aorist_util::AOption;
use aorist_util::{AString, AVec};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;

define_task_node!(
    RPythonTask,
    |task: &RPythonTask| vec![task.call.clone()].into_iter().collect(),
    |task: &RPythonTask| { task.get_native_python_statements() },
    |_task: &RPythonTask| {
        vec![
            PythonImport::PythonModuleImport("rpy2".into(), AOption(ROption::RNone)),
            PythonImport::PythonModuleImport(
                "rpy2.robjects".into(),
                AOption(ROption::RSome("robjects".into())),
            ),
        ]
        .into_iter()
        .collect()
    },
    PythonImport,
    task_val: AST,
    call: AST,
    args: AVec<AST>,
    kwargs: LinkedHashMap<AString, AST>,
    dep_list: AOption<AST>,
    preamble: AOption<AString>,
);
impl PythonFunctionCallTask for RPythonTask {
    fn get_preamble(&self) -> AOption<NativePythonPreamble> {
        let rpy2 = PythonImport::PythonModuleImport("rpy2".into(), AOption(ROption::RNone));
        let rpy2o = PythonImport::PythonModuleImport(
            "rpy2.robjects".into(),
            AOption(ROption::RSome("robjects".into())),
        );
        let body = "
def execute_r(call, preamble=None, **kwargs):
    airflow_args = {
        'ds', 'ds_nodash', 'inlets', 'next_ds',
        'next_ds_nodash', 'outlets', 'prev_ds',
        'prev_ds_nodash', 'run_id', 'task_instance_key_str',
        'tomorrow_ds_nodash', 'ts', 'ts_nodash',
        'ts_nodash_with_tz', 'yesterday_ds',
        'yesterday_ds_nodash', 'test_mode',
        'tomorrow_ds'
    }   
    if preamble is not None:
        robjects.r(preamble)
    return str(robjects.r(
        \"%s(%s)\"
        % (call, ', '.join([
            ('%s = \"%s\"' % (k, v))
            if isinstance(v, str) 
            else (('%s = %s' % (k, str(v)))
                  if isinstance(v, int) or isinstance(v, float)
                  else ('%s = c(%s)' % (k, ', '.join(v)))
                 )
            for k, v in kwargs.items()
            if (
                isinstance(v, str)
                or isinstance(v, float)
                or isinstance(v, int)
                or isinstance(v, tuple)
                or isinstance(v, list)
            ) and not k in airflow_args
        ]))
    ))

";
        AOption(ROption::RSome(NativePythonPreamble {
            imports: vec![rpy2].into_iter().collect(),
            from_imports: vec![rpy2o].into_iter().collect(),
            body: body.into(),
        }))
    }
    fn get_call(&self) -> AST {
        let mut inner_kwargs = LinkedHashMap::new();
        inner_kwargs.insert("call".into(), self.call.clone());
        if let AOption(ROption::RSome(ref p)) = self.preamble {
            inner_kwargs.insert(
                "preamble".into(),
                AST::StringLiteral(StringLiteral::new_wrapped(
                    format!("\n{}\n", p).as_str().into(),
                    false,
                )),
            );
        }
        for (k, v) in self.kwargs.clone() {
            inner_kwargs.insert(k, v);
        }
        AST::Call(Call::new_wrapped(
            AST::SimpleIdentifier(SimpleIdentifier::new_wrapped("execute_r".into())),
            vec![].into_iter().collect(),
            inner_kwargs,
        ))
    }
}
impl PythonTaskBase for RPythonTask {
    fn get_task_val(&self) -> AST {
        self.task_val.clone()
    }
}
