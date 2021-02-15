#![allow(non_snake_case)]
use crate::compliance::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::{AoristConstraint, Constraint};
use crate::dataset::*;
use crate::endpoints::*;
use crate::role::*;
use crate::role_binding::*;
use crate::user::*;
use crate::user_group::*;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct Universe {
    pub name: String,
    #[constrainable]
    pub users: Option<Vec<User>>,
    #[constrainable]
    pub groups: Option<Vec<UserGroup>>,
    #[constrainable]
    pub datasets: Option<Vec<DataSet>>,
    #[constrainable]
    pub role_bindings: Option<Vec<RoleBinding>>,
    #[constrainable]
    pub endpoints: EndpointConfig,
    #[constrainable]
    pub compliance: Option<ComplianceConfig>,
}
pub trait TUniverse {
    fn get_concept_map<'a>(&'a self) -> HashMap<(Uuid, String), Concept<'a>>;
    fn get_constraints_map(
        &self,
        queue: VecDeque<Arc<RwLock<Constraint>>>,
    ) -> HashMap<(Uuid, String), Arc<RwLock<Constraint>>>;
    fn get_user_unixname_map(&self) -> HashMap<String, User>;
    fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String>;
}
impl TUniverse for Universe {
    fn get_concept_map<'a>(&'a self) -> HashMap<(Uuid, String), Concept<'a>> {
        let mut concept_map: HashMap<(Uuid, String), Concept<'a>> = HashMap::new();
        let concept = Concept::Universe((&self, 0, None));
        concept.populate_child_concept_map(&mut concept_map);
        concept_map
    }
    fn get_constraints_map(
        &self,
        mut queue: VecDeque<Arc<RwLock<Constraint>>>,
    ) -> HashMap<(Uuid, String), Arc<RwLock<Constraint>>> {
        let mut constraints_map: HashMap<(Uuid, String), Arc<RwLock<Constraint>>> = HashMap::new();
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
    fn get_user_unixname_map(&self) -> HashMap<String, User> {
        self.users
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| (x.get_unixname().clone(), x.clone()))
            .collect()
    }
    fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String> {
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
