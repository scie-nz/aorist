use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use aorist_primitives::{Ancestry, TAoristObject};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use std::marker::PhantomData;
use aorist_primitives::AoristConcept;
use crate::concept::AoristRef;
use std::collections::HashMap;
use tracing::info;

pub trait SatisfiableConstraint<'a>: TConstraint<'a>
{
    type TAncestry: Ancestry;
    fn satisfy(
        &mut self,
        c: <Self::TAncestry as Ancestry>::TConcept,
        d: &Dialect,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<Option<(String, String, ParameterTuple, Dialect)>>;

    fn satisfy_given_preference_ordering(
        &mut self,
        r: <Self::TAncestry as Ancestry>::TConcept,
        preferences: &Vec<Dialect>,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
// TODO: duplicate function, should be unified in trait
pub trait SatisfiableOuterConstraint<'a>: OuterConstraint<'a>
{
    fn satisfy_given_preference_ordering(
        &mut self,
        c: <<Self as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
        preferences: &Vec<Dialect>,
        ancestry: Arc<<Self as OuterConstraint<'a>>::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
pub trait TBuilder<'a> {
    type OuterType: OuterConstraint<'a, TEnum=Self::EnumType>;
    type EnumType: TConstraintEnum<'a, BuilderT=Self>;
    fn get_constraint_name(&self) -> String;
    fn get_required_constraint_names(&self) -> Vec<String>;
    /*fn build_constraint(
        &self,
        root_uuid: Uuid,
        potential_child_constraints: Vec<Arc<RwLock<ConstraintType>>>,
    ) -> Result<ConstraintType>;*/
}

pub trait TConstraintEnum<'a>
{
    type BuilderT: TBuilder<'a>;
    fn builders() -> Vec<Self::BuilderT>;
    fn get_required_constraint_names() -> HashMap<String, Vec<String>>;
    fn get_explanations() -> HashMap<String, (Option<String>, Option<String>)>;
}
pub trait ConstraintEnum<'a> {}

pub trait OuterConstraint<'a>: TAoristObject + std::fmt::Display
{
    type TEnum: ConstraintEnum<'a> + TConstraintEnum<'a>;
    type TAncestry: Ancestry;

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
pub trait TConstraint<'a>
where
    Self::Root: AoristConcept,
    Self::Outer: OuterConstraint<'a, TAncestry = Self::Ancestry>,
    Self::Ancestry: Ancestry,
{
    type Root;
    type Outer;
    type Ancestry;

    fn get_root_type_name() -> Result<String>;
    fn get_required_constraint_names() -> Vec<String>;
    fn new(
        root_uuid: Uuid,
        potential_child_constraints: Vec<Arc<RwLock<Self::Outer>>>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn should_add(
        root: <<Self as TConstraint<'a>>::Ancestry as Ancestry>::TConcept,
        ancestry: &<Self as TConstraint<'a>>::Ancestry,
    ) -> bool;
}
pub trait ConstraintSatisfactionBase<'a>
where
    Self::RootType: AoristConcept,
    Self::Outer: OuterConstraint<'a>,
    Self::ConstraintType: TConstraint<'a, Root = Self::RootType, Outer = Self::Outer>,
{
    type ConstraintType;
    type RootType;
    type Outer;
}
pub struct ConstraintBuilder<'a, T: TConstraint<'a>>
{
    pub _phantom: PhantomData<T>,
    pub _phantom_lt: PhantomData<&'a ()>,
}
impl<'a, T: TConstraint<'a>> ConstraintBuilder<'a, T>
{
    pub fn build_constraint(
        &self,
        root_uuid: Uuid,
        potential_child_constraints: Vec<Arc<RwLock<T::Outer>>>,
    ) -> Result<T> {
        <T as crate::constraint::TConstraint<'a>>::new(root_uuid, potential_child_constraints)
    }
    pub fn get_root_type_name(&self) -> Result<String> {
        <T as crate::constraint::TConstraint<'a>>::get_root_type_name()
    }
}
