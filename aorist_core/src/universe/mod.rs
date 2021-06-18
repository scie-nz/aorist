use aorist_primitives::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use crate::asset::*;
use crate::compliance::*;
use crate::dataset::*;
use crate::endpoints::*;
use crate::role::*;
use crate::role_binding::*;
use crate::storage::*;
use crate::template::*;
use crate::user::*;
use crate::user_group::*;
use derivative::Derivative;
use linked_hash_map::LinkedHashMap;
use paste::paste;
#[cfg(feature = "python")]
use pyo3::create_exception;
#[cfg(feature = "python")]
use pyo3::exceptions::PyException;
#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;
use serde::{Deserialize, Serialize};
#[cfg(feature = "sql")]
use sqlparser::ast::Statement;
#[cfg(feature = "sql")]
use sqlparser::dialect::GenericDialect;
#[cfg(feature = "sql")]
use sqlparser::parser::Parser;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;
#[cfg(all(feature = "sql", feature = "python"))]
create_exception!(aorist, SQLParseError, PyException);

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
