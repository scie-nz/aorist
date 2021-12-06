use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::role::global_permissions_admin::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::AString;
use aorist_paste::paste;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<AString>;
}

#[enum_dispatch]
#[aorist]
pub enum Role {
    GlobalPermissionsAdmin(AoristRef<GlobalPermissionsAdmin>),
}
