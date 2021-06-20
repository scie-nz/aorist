use crate::concept::{AoristConcept};
use aorist_primitives::Ancestry;
use crate::dialect::Dialect;
use crate::object::TAoristObject;
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use tracing::info;
use uuid::Uuid;

pub trait ConstraintEnum {}
pub trait OuterConstraint<'a>: TAoristObject + std::fmt::Display {
    type TEnum: ConstraintEnum;
    type TAncestry: Ancestry<'a>;

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
pub trait TConstraint<'a, 'b>
where
    Self::Root: AoristConcept<'a>,
    Self::Outer: OuterConstraint<'a, TAncestry = Self::Ancestry>,
    Self::Ancestry: Ancestry<'a>,
    'a: 'b,
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
        root: <<Self as TConstraint<'a, 'b>>::Ancestry as Ancestry<'a>>::TConcept,
        ancestry: &<Self as TConstraint<'a, 'b>>::Ancestry,
    ) -> bool;
}

pub trait ConstraintSatisfactionBase<'a, 'b>
where
    Self::RootType: AoristConcept<'a>,
    Self::Outer: OuterConstraint<'a>,
    Self::ConstraintType: TConstraint<'a, 'b, Root = Self::RootType, Outer = Self::Outer>,
    'a: 'b,
{
    type ConstraintType;
    type RootType;
    type Outer;
}
pub struct ConstraintBuilder<'a, 'b, T: TConstraint<'a, 'b>>
where
    'a: 'b,
{
    pub _phantom: PhantomData<T>,
    pub _phantom_lt: PhantomData<&'a ()>,
    pub _phantom_clt: PhantomData<&'b ()>,
}
impl<'a, 'b, T: TConstraint<'a, 'b>> ConstraintBuilder<'a, 'b, T>
where
    'a: 'b,
{
    pub fn build_constraint(
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
pub trait SatisfiableConstraint<'a, 'b>: TConstraint<'a, 'b>
where
    'a: 'b,
{
    type TAncestry: Ancestry<'a>;
    fn satisfy(
        &mut self,
        c: <Self::TAncestry as Ancestry<'a>>::TConcept,
        d: &Dialect,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<Option<(String, String, ParameterTuple, Dialect)>>;

    fn satisfy_given_preference_ordering(
        &mut self,
        r: <Self::TAncestry as Ancestry<'a>>::TConcept,
        preferences: &Vec<Dialect>,
        ancestry: Arc<Self::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
// TODO: duplicate function, should be unified in trait
pub trait SatisfiableOuterConstraint<'a>: OuterConstraint<'a> {
    fn satisfy_given_preference_ordering(
        &mut self,
        c: <<Self as OuterConstraint<'a>>::TAncestry as Ancestry<'a>>::TConcept,
        preferences: &Vec<Dialect>,
        ancestry: Arc<<Self as OuterConstraint<'a>>::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
