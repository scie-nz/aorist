#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use crate::role::global_permissions_admin::*;
use aorist_concept::{aorist, Constrainable};
use enum_dispatch::enum_dispatch;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[enum_dispatch]
#[aorist]
pub enum Role {
    GlobalPermissionsAdmin(AoristRef<GlobalPermissionsAdmin>),
}
