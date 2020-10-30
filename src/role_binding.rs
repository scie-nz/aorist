#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use crate::role::Role;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleBinding {
    user_name: String,
    role: Role
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
