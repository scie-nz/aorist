#![allow(non_snake_case)]

use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::templates::DatumTemplate;
use crate::assets::Asset;
use crate::ranger::RangerEntity;
use crate::user::User;
use crate::user_group::UserGroup;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSet {
    name: String,
    accessPolicies: Vec<AccessPolicy>,
    datumTemplates: Vec<DatumTemplate>,
    assets: Vec<Asset>,
}
impl DataSet {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_presto_schemas(&self) -> Option<String> {
        Some(self.datumTemplates[0].get_presto_schema())
    }
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Object {
    DataSet(DataSet),
    User(User),
    UserGroup(UserGroup),
}
impl Object {
    pub fn to_yaml(&self) -> String {
        match self {
            Object::DataSet{..} => self.to_yaml(),
            Object::User{..} => self.to_yaml(),
            Object::UserGroup{..} => self.to_yaml(),
        }
    }
}
pub struct DataSetup {
    users: Vec<User>,
    groups: Vec<UserGroup>,
    datasets: Vec<DataSet>,
}
impl DataSetup {
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
    pub fn get_datasets(&self) -> &Vec<DataSet> {
        &self.datasets
    }
    pub fn get_groups(&self) -> &Vec<UserGroup> {
        &self.groups
    }
    pub fn get_curl_calls(
        &self,
        username: String,
        password: String,
        hostname: String,
        port: usize
    ) -> String {
        self.users
            .iter()
            .map(
                |x| x.get_range_create_curl(
                    username.clone(), password.clone(), hostname.clone(), port
                )
            )
            .collect::<Vec<String>>().join("\n")
    }
}
pub fn get_data_setup() -> DataSetup {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let objects: Vec<Object> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    let mut dataSetup = DataSetup{ users: Vec::new(), datasets: Vec::new(), groups: Vec::new() };
    for object in objects {
        match object {
            Object::User(u) => dataSetup.users.push(u),
            Object::DataSet(d) => dataSetup.datasets.push(d),
            Object::UserGroup(g) => dataSetup.groups.push(g),
        }
    }
    dataSetup
}
