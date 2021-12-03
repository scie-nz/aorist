use anyhow::{Context, Result};
#[cfg(feature = "python")]
use aorist_ast::{StringLiteral, AST};
#[cfg(feature = "python")]
use aorist_core::{Ancestry, TOuterProgram};
use aorist_core::{AoristRef, Concept, ConceptAncestry, Dialect, ParameterTuple};
use aorist_core::{
    ConstraintBuilder, ConstraintEnum, ConstraintSatisfactionBase, OuterConstraint, TBuilder,
    TConstraint, TConstraintEnum, TProgram,
};
use aorist_primitives::{define_constraint, register_constraint_new, TAoristObject};
#[cfg(feature = "python")]
use aorist_util::init_logging;
use linked_hash_map::LinkedHashMap;
use maplit::hashmap;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use tracing::debug;

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));
impl<'a> ConstraintEnum<'a> for AoristConstraint {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: String,
    pub root: String,
    pub requires: Option<Vec<String>>,
}
impl<'a> OuterConstraint<'a> for Constraint {
    type TEnum = AoristConstraint;
    type TAncestry = ConceptAncestry;

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
