#![allow(non_snake_case)]
use crate::datasets::DataSet;
use crate::data_setup::DataSetup;
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use serde::{Deserialize, Serialize};

pub trait TAoristObject {
    fn get_name(&self) -> &String;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AoristObject {
    DataSet(DataSet),
    User(User),
    UserGroup(UserGroup),
    RoleBinding(RoleBinding),
    DataSetup(DataSetup),
}
impl AoristObject {
    pub fn to_yaml(&self) -> String {
        match self {
            AoristObject::DataSet { .. } => self.to_yaml(),
            AoristObject::User { .. } => self.to_yaml(),
            AoristObject::UserGroup { .. } => self.to_yaml(),
            AoristObject::RoleBinding { .. } => self.to_yaml(),
            AoristObject::DataSetup { .. } => serde_yaml::to_string(self).unwrap(),
        }
    }
}
