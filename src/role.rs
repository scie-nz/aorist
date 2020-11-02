#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};

pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec![
            "gitea/admin".to_string(),
            "ranger/admin".to_string(),
        ]
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
#[serde(tag = "type", content="spec")]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin)
}
impl TRole for Role {
    fn get_permissions(&self) -> Vec<String> {
        match self {
            Role::GlobalPermissionsAdmin(x) => x.get_permissions()
        }
    }
}
