use crate::constraint::{LiteralsMap, ParameterTuple};
use crate::constraint_state::{ConstraintState, PrefectSingleton};
use crate::prefect::{
    PrefectConstantTaskRender, PrefectProgram, PrefectPythonTaskRender, PrefectRender,
    PrefectShellTaskRender,
};
use aorist_primitives::Dialect;
use linked_hash_set::LinkedHashSet;
use rustpython_parser::ast::Location;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub struct CodeBlock<'a> {
    _dialect: Option<Dialect>,
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    task_render: PrefectRender<'a>,
    constraint_name: String,
}
impl<'a> CodeBlock<'a> {
    pub fn register_literals(&'a self, literals: LiteralsMap) {
        for member in &self.members {
            self.task_render
                .register_literals(literals.clone(), member.clone());
        }
    }
    pub fn new(
        dialect: Option<Dialect>,
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        constraint_name: String,
    ) -> Self {
        let task_render = match dialect {
            Some(Dialect::Python(_)) => PrefectRender::Python(PrefectPythonTaskRender::new(
                members.clone(),
                constraint_name.clone(),
            )),
            Some(Dialect::Bash(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
                constraint_name.clone(),
            )),
            Some(Dialect::Presto(_)) => PrefectRender::Shell(PrefectShellTaskRender::new(
                members.clone(),
                dialect.as_ref().unwrap().clone(),
                constraint_name.clone(),
            )),
            None => PrefectRender::Constant(PrefectConstantTaskRender::new(
                members.clone(),
                constraint_name.clone(),
            )),
            _ => {
                panic!("Dialect not handled: {:?}", dialect.as_ref().unwrap());
            }
        };
        Self {
            _dialect: dialect,
            members,
            task_render,
            constraint_name,
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
    pub fn get_singletons(&'a self, literals: LiteralsMap) -> HashMap<String, PrefectSingleton> {
        let singletons = self
            .task_render
            .get_compressed_singletons(literals, self.constraint_name.clone());
        // TODO: this is very hacky, should dedup by parsing the preambles
        let python_preambles: LinkedHashSet<String> = singletons
            .values()
            .map(|x| {
                if let Some(Dialect::Python(_)) = x.get_dialect() {
                    x.get_preamble()
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        singletons
    }
    pub fn render(&'a self, location: Location, literals: LiteralsMap) {
        for singleton in self.get_singletons(literals).values() {
            println!(
                "{}\n",
                PrefectProgram::render_suite(singleton.as_suite(location),).unwrap()
            );
        }
    }
}
