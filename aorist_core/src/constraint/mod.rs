use crate::dialect::Dialect;
use crate::parameter_tuple::ParameterTuple;
use anyhow::Result;
use aorist_primitives::{Ancestry, OuterConstraint, TConstraint};
use std::sync::Arc;

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
