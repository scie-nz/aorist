#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::role::global_permissions_admin::GlobalPermissionsAdmin;
use aorist_concept::Constrainable;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Constrainable, Hash, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
