use crate::constraint::{LiteralsMap, ParameterTuple};
use crate::constraint_state::ConstraintState;
use crate::prefect::{
    PrefectConstantTaskRender, PrefectPythonTaskRender, PrefectRender, PrefectShellTaskRender,
};
use aorist_primitives::Dialect;
use rustpython_parser::ast::Location;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub struct CodeBlock<'a> {
    _dialect: Option<Dialect>,
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    task_render: PrefectRender<'a>,
}
impl<'a> CodeBlock<'a> {
    pub fn register_literals(&'a self, literals: LiteralsMap) {
        for member in &self.members {
            self.task_render
                .register_literals(literals.clone(), member.clone());
        }
    }
    pub fn new(dialect: Option<Dialect>, members: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> Self {
        let task_render = match dialect {
            Some(Dialect::Python(_)) => {
                PrefectRender::Python(PrefectPythonTaskRender::new(members.clone()))
            }
            Some(Dialect::Bash(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
            )),
            Some(Dialect::Presto(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
            )),
            None => PrefectRender::Constant(PrefectConstantTaskRender::new(members.clone())),
            _ => {
                panic!("Dialect not handled: {:?}", dialect.as_ref().unwrap());
            }
        };
        Self {
            _dialect: dialect,
            members,
            task_render,
        }
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
    pub fn render(&'a self, location: Location, literals: LiteralsMap) {
        self.task_render.render(location, literals);
    }
}
