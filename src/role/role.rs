#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::role::global_permissions_admin::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use enum_dispatch::enum_dispatch;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[enum_dispatch]
#[aorist_concept2]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
