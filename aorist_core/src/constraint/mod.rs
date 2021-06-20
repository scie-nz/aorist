use crate::concept::{AoristConcept};
use aorist_primitives::{Ancestry, ConstraintEnum, TAoristObject, OuterConstraint, TConstraint};
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use tracing::info;
use uuid::Uuid;

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
