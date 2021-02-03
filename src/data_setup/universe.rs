#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::dataset::DataSet;
use crate::endpoints::EndpointConfig;
use crate::role::TRole;
use crate::role_binding::RoleBinding;
use crate::user::{TUser, User};
use crate::user_group::UserGroup;
use aorist_concept::{aorist_concept, Constrainable};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct Universe {
    name: String,
    #[constrainable]
    pub users: Option<Vec<User>>,
    #[constrainable]
    pub groups: Option<Vec<UserGroup>>,
    #[constrainable]
    pub datasets: Option<Vec<DataSet>>,
    #[constrainable]
    pub role_bindings: Option<Vec<RoleBinding>>,
    #[constrainable]
    endpoints: EndpointConfig,
}
impl Universe {
    pub fn get_concept_map<'a>(&'a self) -> HashMap<(Uuid, String), Concept<'a>> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::Universe((&self, 0, None));
        concept.populate_child_concept_map(&mut concept_map);
        concept_map
    }
    pub fn get_constraints_map(&self) -> HashMap<(Uuid, String), Arc<RwLock<Constraint>>> {
        let mut constraints_map: HashMap<(Uuid, String), Arc<RwLock<Constraint>>> = HashMap::new();
        let mut queue: VecDeque<Arc<RwLock<Constraint>>> =
            self.get_constraints().iter().map(|x| x.clone()).collect();
        while let Some(constraint) = queue.pop_front() {
            let uuid = constraint.read().unwrap().get_uuid();
            let root_type = constraint.read().unwrap().root.clone();
            for elem in constraint
                .read()
                .unwrap()
                .get_downstream_constraints_ignore_chains()
            {
                queue.push_back(elem.clone());
            }
            constraints_map.insert((uuid, root_type), constraint);
        }
        constraints_map
    }
    pub fn new(name: String, tag: String, endpoints: EndpointConfig) -> Self {
        Self {
            name: name,
            users: None,
            datasets: None,
            groups: None,
            role_bindings: None,
            endpoints: endpoints,
            constraints: Vec::new(),
            uuid: None,
            tag: Some(tag),
        }
    }
    pub fn get_user_unixname_map(&self) -> HashMap<String, User> {
        self.users.as_ref().unwrap()
            .iter()
            .map(|x| (x.get_unixname().clone(), x.clone()))
            .collect()
    }
    pub fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<_, HashSet<String>> = HashMap::new();
        for binding in self.role_bindings.as_ref().unwrap() {
            let name = binding.get_user_name();
            if !umap.contains_key(name) {
                return Err(format!("Cannot find user with name {}.", name));
            }
            let user = umap.get(name).unwrap().clone();
            for perm in binding.get_role().get_permissions() {
                map.entry(user.get_unixname().clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
}
