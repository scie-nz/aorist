use crate::concept::{Ancestry, TAoristObject, AoristConcept};
use anyhow::Result;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use tracing::info;

pub trait TConstraintEnum {
    type BuilderT;
    fn builders() -> Vec<Self::BuilderT>; 
}
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
