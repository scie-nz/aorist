use crate::constraint::ParameterTuple;
use crate::constraint_state::ConstraintState;
use crate::prefect::{
    PrefectConstantTaskRender, PrefectPythonTaskRender, PrefectShellTaskRender, PrefectTaskRender,
};
use aorist_primitives::Dialect;
use rustpython_parser::ast::Location;
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
    pub fn print_call(&self, location: Location) {
        match &self.dialect {
            Some(Dialect::Python(_)) => {
                PrefectPythonTaskRender::new(self.members.clone()).render(location)
            }
            Some(Dialect::Bash(_)) => {
                PrefectShellTaskRender::new(self.members.clone()).render(location)
            }
            Some(Dialect::Presto(_)) => {
                PrefectShellTaskRender::new(self.members.clone()).render(location)
            }
            None => PrefectConstantTaskRender::new(self.members.clone()).render(location),
            _ => {
                panic!("Dialect not handled: {:?}", self.dialect.as_ref().unwrap());
            }
        }
    }
}
