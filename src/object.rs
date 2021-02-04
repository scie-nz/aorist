#![allow(non_snake_case)]
use crate::constraint::Constraint;
use crate::dataset::DataSet;
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[enum_dispatch(AoristObject)]
pub trait TAoristObject {
    fn get_name(&self) -> &String;
}
#[derive(Serialize, Deserialize)]
pub struct Attribute {}

#[derive(Serialize, Deserialize)]
pub struct Program {}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AoristObject {
    DataSet(DataSet),
    User(User),
    UserGroup(UserGroup),
    RoleBinding(RoleBinding),
    Attribute(Attribute),
    Program(Program),
    Constraint(Constraint),
}
