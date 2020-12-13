#![allow(non_snake_case)]
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use crate::role::global_permissions_admin::GlobalPermissionsAdmin;

#[enum_dispatch(Role)]
pub trait TRole {
    fn get_permissions(&self) -> Vec<String>;
}

#[enum_dispatch]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash)]
#[serde(tag = "type", content = "spec")]
pub enum Role {
    GlobalPermissionsAdmin(GlobalPermissionsAdmin),
}
