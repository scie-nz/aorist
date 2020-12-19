#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::{AoristConstraint, Constraint};
use crate::dataset::DataSet;
use crate::endpoints::EndpointConfig;
use crate::role::TRole;
use crate::role_binding::RoleBinding;
use crate::user::User;
use crate::user_group::UserGroup;
use crate::utils::GetSetError;
use aorist_concept::Constrainable;
use derivative::Derivative;
use getset::{
    Getters, IncompleteGetters, IncompleteMutGetters, IncompleteSetters, MutGetters, Setters,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(
    Serialize,
    Derivative,
    Deserialize,
    Getters,
    Setters,
    MutGetters,
    IncompleteGetters,
    IncompleteSetters,
    IncompleteMutGetters,
    Constrainable,
)]
#[derivative(PartialEq, Debug)]
pub struct ParsedDataSetup {
    name: String,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    #[constrainable]
    users: Option<Vec<User>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    #[constrainable]
    groups: Option<Vec<UserGroup>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    #[constrainable]
    datasets: Option<Vec<DataSet>>,
    #[getset(
        get_incomplete = "pub with_prefix",
        set_incomplete = "pub",
        get_mut_incomplete = "pub with_prefix"
    )]
    #[constrainable]
    role_bindings: Option<Vec<RoleBinding>>,
    //#[constrainable]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    endpoints: EndpointConfig,
    #[getset(get = "pub with_prefix", set = "pub", get_mut = "pub with_prefix")]
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
    uuid: Option<Uuid>,
}
impl ParsedDataSetup {
    pub fn new(name: String, endpoints: EndpointConfig) -> Self {
        Self {
            name: name,
            users: None,
            datasets: None,
            groups: None,
            role_bindings: None,
            endpoints: endpoints,
            constraints: Vec::new(),
            uuid: None,
        }
    }
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
