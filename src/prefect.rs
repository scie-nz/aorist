#![allow(dead_code)]
use crate::constraint::{LiteralsMap, StringLiteral};
use crate::constraint_state::ConstraintState;
use aorist_primitives::Dialect;
use rustpython_parser::ast::{
    Expression, ExpressionType,
    Keyword,
    StringGroup,
};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait PrefectTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>>;
    fn get_constraint_name(&self) -> String;
    fn register_literals(
        &'a self,
        _literals: LiteralsMap,
        _constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
    }
    fn keywords_to_map(&self, keywords: Vec<Keyword>) -> HashMap<String, Expression> {
        assert!(keywords
            .iter()
            .filter(|x| x.name.is_none())
            .next()
            .is_none());
        keywords
            .into_iter()
            .map(|x| (x.name.unwrap(), x.value))
            .collect()
    }
    fn map_to_keywords(&self, map: HashMap<String, Expression>) -> Vec<Keyword> {
        map.into_iter()
            .map(|(k, v)| Keyword {
                name: Some(k),
                value: v,
            })
            .collect()
    }
    fn extract_hashable_value_if_string_constant(&self, expr: &Expression) -> Option<String> {
        match expr.node {
            ExpressionType::String { ref value } => match value {
                StringGroup::Constant { ref value } => Some(value.clone()),
                _ => None,
            },
            _ => None,
        }
    }
    fn render_ids(ids: Vec<Arc<RwLock<ConstraintState<'a>>>>) -> String {
        ids.iter()
            .map(|x| format!("'{}'", x.read().unwrap().get_task_name()).to_string())
            .collect::<Vec<String>>()
            .join(",\n    ")
    }
}
pub struct PrefectPythonTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    constraint_name: String,
}
impl<'a> PrefectTaskRender<'a> for PrefectPythonTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
}
impl<'a> PrefectPythonTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>, constraint_name: String) -> Self {
        Self {
            members,
            constraint_name,
        }
    }
}
pub struct PrefectShellTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    dialect: Dialect,
    constraint_name: String,
}
impl<'a> PrefectTaskRender<'a> for PrefectShellTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
    fn register_literals(&'a self, literals: LiteralsMap, state: Arc<RwLock<ConstraintState<'a>>>) {
        // TODO: this is super hacky, but it should do the job for now
        let read = state.read().unwrap();
        let call = match &self.dialect {
            Dialect::Presto(_) => {
                format!("presto -e '{}'", read.get_call().unwrap().clone()).to_string()
            }
            Dialect::Bash(_) => read.get_call().unwrap().clone(),
            _ => panic!("Unknown dialect encountered for PrefectShellTaskRender."),
        };
        let uuid = read.get_constraint_uuid();

        let mut write = literals.write().unwrap();
        let arc = write
            .entry(call.clone())
            .or_insert(Arc::new(RwLock::new(StringLiteral::new(call))));
        let mut arc_write = arc.write().unwrap();
        arc_write.register_object(uuid, Some("command".to_string()));
        drop(arc_write);
        drop(write);
    }
}
impl<'a> PrefectShellTaskRender<'a> {
    pub fn new(
        members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
        dialect: Dialect,
        constraint_name: String,
    ) -> Self {
        Self {
            members,
            dialect,
            constraint_name,
        }
    }
}
pub struct PrefectConstantTaskRender<'a> {
    members: Vec<Arc<RwLock<ConstraintState<'a>>>>,
    constraint_name: String,
}
impl<'a> PrefectConstantTaskRender<'a> {
    pub fn new(members: Vec<Arc<RwLock<ConstraintState<'a>>>>, constraint_name: String) -> Self {
        Self {
            members,
            constraint_name,
        }
    }
}
impl<'a> PrefectTaskRender<'a> for PrefectConstantTaskRender<'a> {
    fn get_constraints(&self) -> &Vec<Arc<RwLock<ConstraintState<'a>>>> {
        &self.members
    }
    fn get_constraint_name(&self) -> String {
        self.constraint_name.clone()
    }
}

pub enum PrefectRender<'a> {
    Python(PrefectPythonTaskRender<'a>),
    Shell(PrefectShellTaskRender<'a>),
    Constant(PrefectConstantTaskRender<'a>),
}
impl<'a> PrefectRender<'a> {
    pub fn register_literals(
        &'a self,
        literals: LiteralsMap,
        constraint_state: Arc<RwLock<ConstraintState<'a>>>,
    ) {
        //task_render.register_literals(literals.clone(), members.clone());
        match &self {
            PrefectRender::Python(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Shell(x) => x.register_literals(literals, constraint_state),
            PrefectRender::Constant(x) => x.register_literals(literals, constraint_state),
        }
    }
}
