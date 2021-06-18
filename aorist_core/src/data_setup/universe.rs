use crate::compliance::*;
use crate::concept::{AoristConcept, ConceptEnum};
use crate::dataset::DataSet;
use crate::endpoints::*;
use crate::models::Model;
use crate::role::*;
use crate::role_binding::*;
use crate::user::*;
use crate::user_group::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[aorist]
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
    #[constrainable]
    pub models: Option<Vec<Model>>,
}

pub trait TUniverse {
    fn get_user_unixname_map(&self) -> HashMap<String, User>;
    fn get_user_permissions(&self) -> Result<HashMap<String, HashSet<String>>, String>;
}

impl TUniverse for Universe {
    fn get_user_unixname_map(&self) -> HashMap<String, User> {
        self.users
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| (x.unixname.clone(), x.clone()))
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
                map.entry(user.unixname.clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
}