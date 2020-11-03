#![allow(non_snake_case)]
use crate::access_policies::AccessPolicy;
use crate::assets::Asset;
use crate::templates::DatumTemplate;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
//use crate::ranger::RangerEntity;
use crate::object::{AoristObject, TAoristObject};
use crate::role::{Role, TRole};
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use getset::{IncompleteGetters, IncompleteMutGetters, IncompleteSetters};
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
impl TAoristObject for DataSet {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl DataSet {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_mapped_datum_templates(&self) -> HashMap<String, DatumTemplate> {
        self.datumTemplates
            .iter()
            .map(|x| (x.get_name().clone(), x.clone()))
            .collect()
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DataSetup {
    name: String,
    users: Vec<String>,
    groups: Vec<String>,
    datasets: Vec<String>,
    role_bindings: Vec<String>,
}
impl TAoristObject for DataSetup {
    fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize, IncompleteGetters, IncompleteSetters, IncompleteMutGetters)]
pub struct ParsedDataSetup {
    name: String,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    users: Option<Vec<User>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    groups: Option<Vec<UserGroup>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    datasets: Option<Vec<DataSet>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    role_bindings: Option<Vec<RoleBinding>>,
}
impl ParsedDataSetup {
    pub fn get_user_unixname_map(&self) -> HashMap<String, User> {
        let users: &Vec<User> = self.get_users().unwrap();
        users
            .iter()
            .map(|x| (x.get_unixname().clone(), x.clone()))
            .collect()
    }
    pub fn get_user_permissions(&self) -> Result<HashMap<User, HashSet<String>>, String> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<User, HashSet<String>> = HashMap::new();
        for binding in self.get_role_bindings().unwrap() {
            let name = binding.get_user_name();
            if !umap.contains_key(name) {
                return Err(format!("Cannot find user with name {}.", name));
            }
            let user = umap.get(name).unwrap().clone();
            for perm in binding.get_role().get_permissions() {
                map.entry(user.clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
}

impl DataSetup {
    fn parse(self, objects: Vec<AoristObject>) -> ParsedDataSetup {
        let mut dataSetup = ParsedDataSetup {
            name: self.name,
            users: None,
            datasets: None,
            groups: None,
            role_bindings: None,
        };

        let mut users: Vec<User> = Vec::new();
        let mut groups: Vec<UserGroup> = Vec::new();
        let mut datasets: Vec<DataSet> = Vec::new();
        let mut role_bindings: Vec<RoleBinding> = Vec::new();

        for object in objects {
            match object {
                AoristObject::User(u) => users.push(u),
                AoristObject::DataSet(d) => datasets.push(d),
                AoristObject::UserGroup(g) => groups.push(g),
                AoristObject::RoleBinding(r) => role_bindings.push(r),
                _ => {}
            }
        }
        dataSetup.set_users(users).unwrap();
        dataSetup.set_groups(groups).unwrap();
        dataSetup.set_datasets(datasets).unwrap();
        dataSetup.set_role_bindings(role_bindings).unwrap();

        let mut role_map: HashMap<String, Vec<Role>> = HashMap::new();
        for binding in dataSetup.get_role_bindings().unwrap() {
            role_map
                .entry(binding.get_user_name().clone())
                .or_insert_with(Vec::new)
                .push(binding.get_role().clone());
        }
        let mut user_map: HashMap<String, &mut User> = HashMap::new();

        for user in dataSetup.get_users_mut().unwrap().iter_mut() {
            let username = user.get_unixname();
            if role_map.contains_key(username) {
                user_map.insert(username.clone(), user);
            } else {
                user.set_roles(Vec::new()).unwrap();
            }
        }
        for (user_name, roles) in role_map.into_iter() {
            user_map
                .get_mut(&user_name)
                .unwrap()
                .set_roles(roles)
                .unwrap();
        }
        dataSetup
    }
}

pub fn get_data_setup() -> ParsedDataSetup {
    let s = fs::read_to_string("basic.yaml").unwrap();
    let objects: Vec<AoristObject> = s
        .split("\n---\n")
        .filter(|x| x.len() > 0)
        .map(|x| serde_yaml::from_str(x).unwrap())
        .collect();
    let v: Vec<Option<&DataSetup>> = objects
        .iter()
        .map(|x| match x {
            AoristObject::DataSetup(x) => Some(x),
            _ => None,
        })
        .filter(|x| x.is_some())
        .collect();
    let dataSetup: DataSetup = v.first().unwrap().unwrap().to_owned();
    dataSetup.parse(objects)
}
