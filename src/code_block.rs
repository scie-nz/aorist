use crate::constraint::ParameterTuple;
use crate::constraint_state::ConstraintState;
use crate::prefect::{
    PrefectConstantTaskRender, PrefectPythonTaskRender, PrefectShellTaskRender,
    PrefectTaskRenderWithCalls,
};
use aorist_primitives::Dialect;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
pub struct CodeBlock<'a> {
    dialect: Option<Dialect>,
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
}
impl<'a> CodeBlock<'a> {
    pub fn new(dialect: Option<Dialect>, members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        Self { dialect, members }
    }
    pub fn get_preambles(&self) -> HashSet<String> {
        self.members
            .iter()
            .map(|x| x.read().unwrap().get_preamble())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
    pub fn get_params(&self) -> HashMap<String, Option<ParameterTuple>> {
        self.members
            .iter()
            .map(|rw| {
                let x = rw.read().unwrap();
                (x.get_task_name(), x.get_params())
            })
            .collect()
    }
    // TODO: move this to Dialect class
    pub fn print_call(&self, constraint_name: String) {
        match &self.dialect {
            Some(Dialect::Python(_)) => {
                PrefectPythonTaskRender::new(self.members.clone()).render(constraint_name)
            }
            Some(Dialect::Bash(_)) => {
                PrefectShellTaskRender::new(self.members.clone()).render(constraint_name)
            }
            Some(Dialect::Presto(_)) => {
                PrefectShellTaskRender::new(self.members.clone()).render(constraint_name)
            }
            None => PrefectConstantTaskRender::new(self.members.clone()).render(constraint_name),
            _ => {
                panic!("Dialect not handled: {:?}", self.dialect.as_ref().unwrap());
            }
        }
    }
}
