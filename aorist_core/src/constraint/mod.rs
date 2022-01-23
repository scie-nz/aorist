use crate::parameter_tuple::ParameterTuple;
use abi_stable::external_types::parking_lot::rw_lock::RRwLock;
use abi_stable::std_types::RArc;
use anyhow::Result;
use aorist_primitives::Dialect;
use aorist_primitives::{Ancestry, AoristConcept, TAoristObject, ToplineConcept};
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec};
use scienz::AoristError;
use std::collections::HashMap;
use std::marker::PhantomData;
use tracing::info;

use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RResult, RString, RVec},
    StableAbi,
};

pub trait SatisfiableConstraint<'a>: TConstraint<'a> {
    type TAncestry: Ancestry;
    fn satisfy(
        &mut self,
        c: <Self::TAncestry as Ancestry>::TConcept,
        d: &Dialect,
        ancestry: RArc<Self::TAncestry>,
    ) -> Result<AOption<(AString, AString, ParameterTuple, Dialect)>>;

    fn satisfy_given_preference_ordering(
        &mut self,
        r: <Self::TAncestry as Ancestry>::TConcept,
        preferences: &AVec<Dialect>,
        ancestry: RArc<Self::TAncestry>,
    ) -> Result<(AString, AString, ParameterTuple, Dialect)>;
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = "ConstraintMod_Ref")))]
#[sabi(missing_field(panic))]
pub struct ConstraintMod {
    #[sabi(last_prefix_field)]
    pub new: extern "C" fn() -> RResult<RString, AoristError>,
    pub builders: extern "C" fn() -> RResult<RVec<RString>, AoristError>,
}

impl RootModule for ConstraintMod_Ref {
    declare_root_module_statics! {ConstraintMod_Ref}
    const BASE_NAME: &'static str = "constraint";
    const NAME: &'static str = "constraint";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

// TODO: duplicate function, should be unified in trait
pub trait SatisfiableOuterConstraint<'a>: OuterConstraint<'a> {
    fn satisfy_given_preference_ordering(
        &mut self,
        c: <<Self as OuterConstraint<'a>>::TAncestry as Ancestry>::TConcept,
        preferences: &AVec<Dialect>,
        ancestry: RArc<<Self as OuterConstraint<'a>>::TAncestry>,
    ) -> Result<(AString, AString, ParameterTuple, Dialect)>;
}
pub trait TBuilder<'a> {
    type TEnum: ToplineConcept;
    type TAncestry: Ancestry;
    type OuterType: OuterConstraint<'a>; //, TEnum=Self::EnumType>;
                                         //type EnumType: TConstraintEnum<'a, BuilderT=Self>;
    fn builders() -> AVec<Self>
    where
        Self: Sized;
    fn get_constraint_name(&self) -> AString;
    fn get_required_constraint_names(&self) -> AVec<AString>;
    fn build_constraint(
        &self,
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<Self::OuterType>>>,
    ) -> Result<Self::OuterType>;
    fn get_root_type_name(&self) -> Result<AString>;
    fn get_required(&self, root: Self::TEnum, ancestry: &Self::TAncestry) -> AVec<AUuid>;
    fn should_add(&self, root: Self::TEnum, ancestry: &Self::TAncestry) -> bool;
}

pub trait TConstraintEnum<'a>: Sized + Clone {
    fn get_required_constraint_names() -> HashMap<AString, AVec<AString>>;
    fn get_explanations() -> HashMap<AString, (AOption<AString>, AOption<AString>)>;
    #[cfg(feature = "python")]
    fn get_py_obj<'b>(&self, py: pyo3::Python<'b>) -> pyo3::prelude::PyObject;
}
pub trait ConstraintEnum<'a> {}

pub trait OuterConstraint<'a>: TAoristObject + std::fmt::Display + Clone {
    type TEnum: Sized + ConstraintEnum<'a> + TConstraintEnum<'a>;
    type TAncestry: Ancestry;

    fn get_uuid(&self) -> Result<AUuid>;
    fn get_root(&self) -> AString;
    fn get_root_uuid(&self) -> Result<AUuid>;
    fn get_downstream_constraints(&self) -> Result<AVec<RArc<RRwLock<Self>>>>;
    fn requires_program(&self) -> Result<bool>;
    fn get_root_type_name(&self) -> Result<AString>;
    fn print_dag(&self) -> Result<()> {
        for downstream_rw in self.get_downstream_constraints()? {
            let downstream = downstream_rw.read();
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
            let downstream = downstream_rw.read();
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

    fn get_root_type_name() -> Result<AString>;
    fn get_required_constraint_names() -> AVec<AString>;
    fn new(
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<Self::Outer>>>,
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
pub struct ConstraintBuilder<'a, T: TConstraint<'a>> {
    pub _phantom: PhantomData<T>,
    pub _phantom_lt: PhantomData<&'a ()>,
}
impl<'a, T: TConstraint<'a>> ConstraintBuilder<'a, T> {
    pub fn build_constraint(
        &self,
        root_uuid: AUuid,
        potential_child_constraints: AVec<RArc<RRwLock<T::Outer>>>,
    ) -> Result<T> {
        <T as crate::constraint::TConstraint<'a>>::new(root_uuid, potential_child_constraints)
    }
    pub fn get_root_type_name(&self) -> Result<AString> {
        <T as crate::constraint::TConstraint<'a>>::get_root_type_name()
    }
}
