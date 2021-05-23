#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, ConceptEnum, Concept};
use crate::role::global_permissions_admin::*;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use enum_dispatch::enum_dispatch;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[enum_dispatch]
#[aorist_concept]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
