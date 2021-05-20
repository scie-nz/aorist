use crate::concept::{AoristConcept, Concept, ConceptAncestry, Ancestry};
use crate::dialect::Dialect;
use crate::object::TAoristObject;
use crate::parameter_tuple::ParameterTuple;
use anyhow::{Context, Result};
use aorist_primitives::{define_constraint, register_constraint};
use maplit::hashmap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use tracing::info;

pub struct ConstraintBuilder<'a, 'b, T: TConstraint<'a, 'b>> where 'a : 'b {
    _phantom: PhantomData<T>,
    _phantom_lt: PhantomData<&'a ()>,
    _phantom_clt: PhantomData<&'b ()>,
}
impl<'a, 'b, T: TConstraint<'a, 'b>> ConstraintBuilder<'a, 'b, T> where 'a : 'b {
    fn build_constraint(
        &self,
        root_uuid: Uuid,
        potential_child_constraints: Vec<Arc<RwLock<T::Outer>>>,
    ) -> Result<T> {
        <T as crate::constraint::TConstraint<'a, 'b>>::new(root_uuid, potential_child_constraints)
    }
    pub fn get_root_type_name(&self) -> Result<String> {
        <T as crate::constraint::TConstraint<'a, 'b>>::get_root_type_name()
    }
}

pub trait TConstraint<'a, 'b>
where
    Self::Root: AoristConcept,
    Self::Outer: OuterConstraint,
    'a : 'b
{
    type Root;
    type Outer;

    fn get_root_type_name() -> Result<String>;
    fn get_required_constraint_names() -> Vec<String>;
    fn new(
        root_uuid: Uuid,
        potential_child_constraints: Vec<Arc<RwLock<Self::Outer>>>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn should_add(root: Concept<'a>, ancestry: &ConceptAncestry<'a>) -> bool;
}
pub trait ConstraintSatisfactionBase<'a, 'b>
where
    Self::RootType: AoristConcept,
    Self::ConstraintType: TConstraint<'a, 'b, Root = Self::RootType, Outer=Constraint>,
    'a : 'b,
{
    type ConstraintType;
    type RootType;
}

pub trait SatisfiableConstraint<'a, 'b>: TConstraint<'a, 'b> where 'a : 'b {
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

pub trait ConstraintEnum {}
include!(concat!(env!("OUT_DIR"), "/constraints.rs"));
impl ConstraintEnum for AoristConstraint {}

pub trait OuterConstraint: TAoristObject + std::fmt::Display {
    type TEnum: ConstraintEnum;
    fn get_uuid(&self) -> Result<Uuid>;
    fn get_root(&self) -> String;
    fn get_root_uuid(&self) -> Result<Uuid>;
    fn get_downstream_constraints(&self) -> Result<Vec<Arc<RwLock<Self>>>>;
    fn requires_program(&self) -> Result<bool>;
    fn get_root_type_name(&self) -> Result<String>;
    fn print_dag(&self) -> Result<()> {
        for downstream_rw in self.get_downstream_constraints()? {
            let downstream = downstream_rw.read().unwrap();
            info!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                self.get_name(),
                self.get_root(),
                self.get_root_uuid()?,
                self.get_uuid()?,
                downstream,
                downstream.get_root(),
                downstream.get_root_uuid()?,
                downstream.get_uuid()?,
            );
        }
        for downstream_rw in self.get_downstream_constraints()? {
            let downstream = downstream_rw.read().unwrap();
            downstream.print_dag()?;
        }
        Ok(())
    }
    fn inner(&self, caller: &str) -> Result<&Self::TEnum>;
}

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
