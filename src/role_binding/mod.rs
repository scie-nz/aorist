#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::object::TAoristObject;
use crate::role::Role;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable)]
pub struct RoleBinding {
    user_name: String,
    #[constrainable]
    role: Role,
    name: String,
    uuid: Option<Uuid>,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &String {
        &self.name
    }
}
impl RoleBinding {
    pub fn get_user_name(&self) -> &String {
        &self.user_name
    }
    pub fn get_role(&self) -> &Role {
        &self.role
    }
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
