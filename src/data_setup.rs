#![allow(non_snake_case)]
use crate::datasets::DataSet;
use crate::object::{AoristObject, TAoristObject};
use crate::role::{Role, TRole};
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use getset::{Getters, IncompleteGetters, IncompleteMutGetters, IncompleteSetters};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use crate::imports::local_import::LocalFileImport;
use crate::imports::TAoristImport;
use crate::endpoints::EndpointConfig;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum GetSetError {
    #[error("Get was called, but attribute was not set: {0:#?}")]
    GetError(String),
    #[error("Set was called twice for the attribute: {0:#?}")]
    SetError(String),
}
#[derive(Serialize, Deserialize, Clone, Getters)]
pub struct DataSetup {
    name: String,
    users: Vec<String>,
    groups: Vec<String>,
    datasets: Vec<String>,
    role_bindings: Vec<String>,
    endpoints: EndpointConfig,
    #[getset(get = "pub")]
    imports: Option<Vec<LocalFileImport>>,
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
    endpoints: EndpointConfig,
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
    pub fn get_pipelines(&self) -> Result<HashMap<String, String>, String> {
        let mut files: HashMap<String, String> = HashMap::new();
        for dataset in self.get_datasets().unwrap() {
            files.insert(
                dataset.get_materialize_pipeline_name(),
                dataset.get_materialize_pipeline(&self.endpoints)?,
            );
        }
        Ok(files)
    }
}

impl DataSetup {
    pub fn parse(self, mut objects: Vec<AoristObject>) -> ParsedDataSetup {
        println!("{}", self.imports.is_some());
        if let Some(imports) = self.imports {
            for import in imports {
                for object in import.get_objects().into_iter() {
                    objects.push(object);
                }
            }
        }

        let mut dataSetup = ParsedDataSetup {
            name: self.name,
            users: None,
            datasets: None,
            groups: None,
            role_bindings: None,
            endpoints: self.endpoints,
        };

        let user_names: HashSet<String> = self.users.iter().map(|x| x.clone()).collect();
        let dataset_names: HashSet<String> = self.datasets.iter().map(|x| x.clone()).collect();
        let group_names: HashSet<String> = self.groups.iter().map(|x| x.clone()).collect();
        let role_binding_names: HashSet<String> =
            self.role_bindings.iter().map(|x| x.clone()).collect();

        let mut users: Vec<User> = Vec::new();
        let mut groups: Vec<UserGroup> = Vec::new();
        let mut datasets: Vec<DataSet> = Vec::new();
        let mut role_bindings: Vec<RoleBinding> = Vec::new();

        for object in objects {
            match object {
                AoristObject::User(u) => {
                    if user_names.contains(u.get_name()) {
                        users.push(u)
                    }
                }
                AoristObject::DataSet(d) => {
                    if dataset_names.contains(d.get_name()) {
                        datasets.push(d)
                    }
                }
                AoristObject::UserGroup(g) => {
                    if group_names.contains(g.get_name()) {
                        groups.push(g)
                    }
                }
                AoristObject::RoleBinding(r) => {
                    if role_binding_names.contains(r.get_name()) {
                        role_bindings.push(r)
                    }
                }
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
