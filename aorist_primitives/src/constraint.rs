use crate::concept::{Ancestry, AoristConcept, TAoristObject};
use anyhow::Result;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use tracing::info;
use uuid::Uuid;

pub trait TConstraintEnum<'a, 'b>
where
    'a: 'b,
{
    type BuilderT;
    fn builders() -> Vec<Self::BuilderT>;
}
pub trait ConstraintEnum<'b> {}

pub trait OuterConstraint<'a, 'b>: TAoristObject + std::fmt::Display
where
    'a: 'b,
{
    type TEnum: ConstraintEnum<'b> + TConstraintEnum<'a, 'b>;
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
    Self::Outer: OuterConstraint<'a, 'b, TAncestry = Self::Ancestry>,
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
    Self::Outer: OuterConstraint<'a, 'b>,
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
