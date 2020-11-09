#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
#[serde(tag = "type", content = "spec")]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
