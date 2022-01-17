use crate::role::global_permissions_admin::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> AVec<AString>;
}

#[enum_dispatch]
#[aorist]
pub enum Role {
    GlobalPermissionsAdmin(AoristRef<GlobalPermissionsAdmin>),
}
