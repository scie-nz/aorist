use crate::concept::{Ancestry, AoristConcept, Concept, ConceptAncestry};
use crate::dialect::Dialect;
use crate::object::TAoristObject;
use crate::parameter_tuple::ParameterTuple;
use anyhow::{Context, Result};
pub use aorist_core::{ConstraintEnum, OuterConstraint, TConstraint, ConstraintSatisfactionBase, ConstraintBuilder};
use aorist_primitives::{define_constraint, register_constraint};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};


pub trait SatisfiableConstraint<'a, 'b>: TConstraint<'a, 'b>
where
    'a: 'b,
{
    type TAncestry: Ancestry<'a>;
    fn satisfy(
        &mut self,
        c: Concept<'a>,
        d: &Dialect,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<Option<(String, String, ParameterTuple, Dialect)>>;

    fn satisfy_given_preference_ordering(
        &mut self,
        r: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
// TODO: duplicate function, should be unified in trait
pub trait AllConstraintsSatisfiability {
    fn satisfy_given_preference_ordering<'a>(
        &mut self,
        c: Concept<'a>,
        preferences: &Vec<Dialect>,
        ancestry: Arc<ConceptAncestry<'a>>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));
impl ConstraintEnum for AoristConstraint {}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: String,
    pub root: String,
    pub requires: Option<Vec<String>>,
}
impl OuterConstraint for Constraint {
    type TEnum = AoristConstraint;
    fn get_uuid(&self) -> Result<Uuid> {
        self.inner("get_uuid()")?.get_uuid()
    }
    fn get_root(&self) -> String {
        self.root.clone()
    }
    fn get_root_uuid(&self) -> Result<Uuid> {
        self.inner("get_root_uuid()")?.get_root_uuid()
    }
    fn get_downstream_constraints(&self) -> Result<Vec<Arc<RwLock<Self>>>> {
        self.inner("get_downstream_constraints()")?
            .get_downstream_constraints()
    }
    fn requires_program(&self) -> Result<bool> {
        self.inner("requires_program()")?.requires_program()
    }
    fn get_root_type_name(&self) -> Result<String> {
        self.inner("get_root_type_name()")?.get_root_type_name()
    }
    fn inner(&self, caller: &str) -> Result<&Self::TEnum> {
        self.inner.as_ref().with_context(|| {
            format!(
                "Called {} on Constraint struct with no inner: name={}, root={}",
                caller, self.name, self.root
            )
        })
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
