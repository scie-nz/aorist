use crate::concept::{AoristConcept, Concept, ConceptAncestry};
use crate::object::TAoristObject;
use aorist_primitives::{define_constraint, register_constraint, Dialect};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use rustpython_parser::ast::{Expression, ExpressionType, Location,
StringGroup, Located};

#[derive(Clone)]
pub struct ParameterTuple {
    args: Vec<String>,
}
impl ParameterTuple {
    pub fn new(args: Vec<String>) -> Self {
        Self{ args }
    }
    pub fn get_args_literals(&self, location: Location) -> Vec<Expression> {
        let args = self
            .args
            .iter()
            .map(|x| {
                Located {
                    location,
                    // TODO: this is where other kinds of arguments can live
                    node: ExpressionType::String {
                        value: StringGroup::Constant {
                            value: x.to_string(),
                        },
                    },
                }
            })
            .collect::<Vec<_>>();
        args
    }
}

pub trait TConstraint
where
    Self::Root: AoristConcept,
{
    type Root;
    fn get_root_type_name() -> String;
    fn get_required_constraint_names() -> Vec<String>;
}
pub trait ConstraintSatisfactionBase
where
    Self::RootType: AoristConcept,
    Self::ConstraintType: TConstraint<Root = Self::RootType>,
{
    type ConstraintType;
    type RootType;
}

pub trait SatisfiableConstraint: TConstraint {
    fn satisfy<'a>(
        &self,
        c: Concept<'a>,
        d: &Dialect,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Option<(String, String, ParameterTuple)>;

    fn satisfy_given_preference_ordering<'a>(
        &self,
        r: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Result<(String, String, ParameterTuple, Dialect), String>;
}
// TODO: duplicate function, should be unified in trait
pub trait AllConstraintsSatisfiability {
    fn satisfy_given_preference_ordering<'a>(
        &self,
        c: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Result<(String, String, ParameterTuple, Dialect), String>;
}

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: String,
    pub root: String,
    pub requires: Option<Vec<String>>,
}
impl Constraint {
    pub fn get_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_uuid();
        }
        panic!("Called get_uuid() on a Constraint struct with no inner");
    }
    pub fn get_root(&self) -> String {
        self.root.clone()
    }
    pub fn get_root_uuid(&self) -> Uuid {
        if let Some(c) = &self.inner {
            return c.get_root_uuid();
        }
        panic!("Called get_root_uuid() on a Constraint struct with no inner");
    }
    pub fn get_downstream_constraints(&self) -> Vec<Arc<RwLock<Constraint>>> {
        if let Some(c) = &self.inner {
            return c.get_downstream_constraints();
        }
        panic!("Called get_downstream_constraints() on a Constraint struct with no inner");
    }
    pub fn get_downstream_constraints_ignore_chains(&self) -> Vec<Arc<RwLock<Constraint>>> {
        if let Some(c) = &self.inner {
            return c.get_downstream_constraints_ignore_chains();
        }
        panic!("Called get_downstream_constraints() on a Constraint struct with no inner");
    }
    pub fn ingest_upstream_constraints(
        &mut self,
        upstream_constraints: Vec<Arc<RwLock<Constraint>>>,
    ) {
        if let Some(ref mut c) = &mut self.inner {
            return c.ingest_upstream_constraints(upstream_constraints);
        }
        panic!("Called ingest_upstream_constraints() on a Constraint struct with no inner");
    }
    pub fn requires_program(&self) -> bool {
        if let Some(ref c) = &self.inner {
            return c.requires_program();
        }
        panic!("Called requires_program() on a Constraint struct with no inner");
    }
    pub fn print_dag(&self) {
        for downstream_rw in self.get_downstream_constraints() {
            let downstream = downstream_rw.read().unwrap();
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.get_name(),
                self.get_root(),
                self.get_root_uuid(),
                self.get_uuid(),
                downstream,
                downstream.get_root(),
                downstream.get_root_uuid(),
                downstream.get_uuid(),
            );
        }
        for downstream_rw in self.get_downstream_constraints() {
            let downstream = downstream_rw.read().unwrap();
            downstream.print_dag();
        }
    }
}
impl TAoristObject for Constraint {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
