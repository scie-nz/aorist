#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::role::*;
use aorist_concept::{aorist_concept2, ConstrainObject, Constrainable};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub struct RoleBinding {
    user_name: String,
    #[constrainable]
    role: Role,
    name: String,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub trait TRoleBinding {
    fn get_user_name(&self) -> &String;
    fn get_role(&self) -> &Role;
    fn to_yaml(&self) -> String;
}
impl TRoleBinding for RoleBinding {
    fn get_user_name(&self) -> &String {
        &self.user_name
    }
    fn get_role(&self) -> &Role {
        &self.role
    }
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
