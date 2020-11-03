#![allow(non_snake_case)]
use crate::access_policies::AccessPolicy;
use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::{HashMap, HashSet};
use crate::templates::DatumTemplate;
use crate::assets::Asset;
//use crate::ranger::RangerEntity;
use crate::role::{Role, TRole};
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use getset::{IncompleteGetters, IncompleteSetters};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum GetSetError {
    #[error("Get was called, but attribute was not set: {0:#?}")]
    GetError(String),
    #[error("Set was called twice for the attribute: {0:#?}")]
    SetError(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
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
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates.iter().map(|x| (x.get_name().clone(), x.clone())).collect()
    }
    pub fn get_presto_schemas(&self, indent: usize) -> String {
        let mappedTemplates = self.get_mapped_datum_templates();
        let mut schemas: String = "".to_string();
        for asset in &self.assets {
            let schema = asset.get_presto_schemas(&mappedTemplates, indent);
            schemas += "\n\n";
            schemas += &schema;
        }
        schemas
    }
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Object {
    DataSet(DataSet),
    User(User),
    UserGroup(UserGroup),
    RoleBinding(RoleBinding),
}
impl Object {
    pub fn to_yaml(&self) -> String {
        match self {
            Object::DataSet{..} => self.to_yaml(),
            Object::User{..} => self.to_yaml(),
            Object::UserGroup{..} => self.to_yaml(),
            Object::RoleBinding{..} => self.to_yaml(),
        }
    }
}
#[derive(IncompleteGetters, IncompleteSetters)]
pub struct DataSetup2 {
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    users: Option<Vec<User>>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    groups: Option<Vec<UserGroup>>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    datasets: Option<Vec<DataSet>>,
    #[getset(get_incomplete = "pub", set_incomplete = "pub")]
    role_bindings: Option<Vec<RoleBinding>>,
}

pub struct DataSetup {
    users: Vec<User>,
    groups: Vec<UserGroup>,
    datasets: Vec<DataSet>,
    role_bindings: Vec<RoleBinding>,
}
impl DataSetup {
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
    pub fn get_mutable_users(&mut self) -> &mut Vec<User> {
        &mut self.users
    }
    pub fn get_datasets(&self) -> &Vec<DataSet> {
        &self.datasets
    }
    pub fn get_groups(&self) -> &Vec<UserGroup> {
        &self.groups
    }
    pub fn get_role_bindings(&self) -> &Vec<RoleBinding> {
        &self.role_bindings
    }
    pub fn get_user_unixname_map(&self) -> HashMap<String, User>  {
        self.get_users().iter().map(|x| (x.get_unixname().clone(), x.clone())).collect()
    }
    pub fn get_user_permissions(&self) -> Result<HashMap<User, HashSet<String>>, String> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<User, HashSet<String>> = HashMap::new();
        for binding in self.get_role_bindings() {
            let name = binding.get_user_name();
            if !umap.contains_key(name) {
                return Err(format!("Cannot find user with name {}.", name));
            }
            let user = umap.get(name).unwrap().clone();
            for perm in binding.get_role().get_permissions() {
                map.entry(user.clone()).or_insert_with(HashSet::new).insert(perm.clone());
            }
        }
        Ok(map)
    }
    /*
    pub fn get_gitea_user_update_calls(&self) {
    }
    pub fn get_curl_calls(
        &self,
        username: String,
        password: String,
        hostname: String,
        port: usize
    ) -> String {
        format!(
            "{}\n\n{}",
            self.groups
                .iter()
                .map(
                    |x| x.get_range_create_curl(
                        username.clone(), password.clone(), hostname.clone(), port
                    )
                )
                .collect::<Vec<String>>().join("\n"),
            self.users
                .iter()
                .map(
                    |x| x.get_range_create_curl(
                        username.clone(), password.clone(), hostname.clone(), port
                    )
                )
                .collect::<Vec<String>>().join("\n")
        )
    }*/
}
pub fn get_data_setup() -> DataSetup {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let objects: Vec<Object> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    let mut dataSetup = DataSetup{
        users: Vec::new(),
        datasets: Vec::new(),
        groups: Vec::new(),
        role_bindings: Vec::new(),
    };
    for object in objects {
        match object {
            Object::User(u) => dataSetup.users.push(u),
            Object::DataSet(d) => dataSetup.datasets.push(d),
            Object::UserGroup(g) => dataSetup.groups.push(g),
            Object::RoleBinding(r) => dataSetup.role_bindings.push(r),
        }
    }
    let mut role_map: HashMap<String, Vec<Role>> = HashMap::new();
    for binding in &dataSetup.role_bindings {
        role_map.entry(binding.get_user_name().clone()).or_insert_with(Vec::new).push(binding.get_role().clone());
    }
    let mut user_map: HashMap<String, &mut User> = HashMap::new();
    for user in dataSetup.users.iter_mut() {
        let username = user.get_unixname();
        if role_map.contains_key(username) {
            user_map.insert(username.clone(), user);
        } else {
            user.set_roles(Vec::new()).unwrap();
        }
    }
    for (user_name, roles) in role_map.into_iter() {
        user_map.get_mut(&user_name).unwrap().set_roles(roles).unwrap();
    }
    //dataSetup.set_optional_assets(dataSetup.datasets.iter().clone().collect());
    dataSetup
}
