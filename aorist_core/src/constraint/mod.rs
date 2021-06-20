use crate::concept::{AoristConcept};
use aorist_primitives::{Ancestry, ConstraintEnum, TAoristObject, OuterConstraint, TConstraint};
use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use tracing::info;
use uuid::Uuid;

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
pub trait SatisfiableOuterConstraint<'a, 'b>: OuterConstraint<'a, 'b> where 'a : 'b {
    fn satisfy_given_preference_ordering(
        &mut self,
        c: <<Self as OuterConstraint<'a, 'b>>::TAncestry as Ancestry<'a>>::TConcept,
        preferences: &Vec<Dialect>,
        ancestry: Arc<<Self as OuterConstraint<'a, 'b>>::TAncestry>,
    ) -> Result<(String, String, ParameterTuple, Dialect)>;
}
