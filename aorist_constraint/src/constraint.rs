use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::{RArc, ROption};
use anyhow::{Context, Result};
#[cfg(feature = "python")]
use aorist_ast::{StringLiteral, AST};
#[cfg(feature = "python")]
use aorist_primitives::{Ancestry};
use aorist_core::{TOuterProgram, Dialect, ParameterTuple}; 
use scienz::{Concept, ConceptAncestry};
use aorist_core::{
    ConstraintBuilder, ConstraintSatisfactionBase, OuterConstraint, TBuilder,
    TConstraint, TConstraintEnum, TProgram,
};
use aorist_primitives::{
    define_constraint, register_constraint_new, AoristConceptBase, 
};
#[cfg(feature = "python")]
use aorist_util::init_logging;
use aorist_util::{AOption, AString, AVec, AoristRef, ATaskId};
use linked_hash_map::LinkedHashMap;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use tracing::debug;

include!(concat!(env!("OUT_DIR"), "/constraints.rs"));

#[derive(Serialize, Deserialize, Clone)]
pub struct Constraint {
    #[serde(skip)]
    pub inner: Option<AoristConstraint>,
    pub name: AString,
    pub root: AString,
    pub requires: Option<AVec<AString>>,
}
impl OuterConstraint for Constraint {
    type TEnum = AoristConstraint;
    type TAncestry = ConceptAncestry;

    fn get_uuid(&self) -> AUuid {
        self.inner("get_uuid()".into()).get_uuid().unwrap()
    }
    fn get_root(&self) -> AString {
        self.root.clone()
    }
    fn get_root_uuid(&self) -> AUuid {
        self.inner("get_root_uuid()".into()).get_root_uuid().unwrap()
    }
    fn get_dependencies(&self) -> AVec<ATaskId> {
        self.inner("get_dependencies()".into())
            .get_dependencies().into_iter().collect()
    }
    fn requires_program(&self) -> bool {
        self.inner("requires_program()".into()).requires_program().unwrap()
    }
    fn get_root_type_name(&self) -> AString {
        self.inner("get_root_type_name()".into()).get_root_type_name().unwrap()
    }
    fn inner(&self, caller: AString) -> &Self::TEnum {
        self.inner.as_ref().with_context(|| {
            format!(
                "Called {} on Constraint struct with no inner: name={}, root={}",
                caller, self.name, self.root
            )
        }).unwrap()
    }
    fn get_name(&self) -> AString {
        self.name.clone()
    }
}
impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.as_str())
    }
}
