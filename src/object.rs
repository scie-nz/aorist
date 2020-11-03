#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use crate::datasets::{DataSet, DataSetup};

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
            AoristObject::DataSet{..} => self.to_yaml(),
            AoristObject::User{..} => self.to_yaml(),
            AoristObject::UserGroup{..} => self.to_yaml(),
            AoristObject::RoleBinding{..} => self.to_yaml(),
            AoristObject::DataSetup{..} => serde_yaml::to_string(self).unwrap(),
        }
    }
}
