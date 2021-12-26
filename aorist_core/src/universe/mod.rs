use crate::compliance::*;
use crate::concept::{AoristRef, WrappedConcept};
use crate::dataset::*;
use crate::endpoints::*;
use crate::role::*;
use crate::role_binding::*;
use crate::user::*;
use crate::user_group::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::{AoristConceptBase, AoristConcept};
use aorist_primitives::ConceptEnum;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use uuid::Uuid;

#[cfg(all(feature = "sql", feature = "python"))]
use pyo3::create_exception;
#[cfg(all(feature = "sql", feature = "python"))]
create_exception!(aorist, SQLParseError, pyo3::exceptions::PyException);

#[aorist]
pub struct Universe {
    pub name: AString,
    #[constrainable]
    pub users: AOption<AVec<AoristRef<User>>>,
    #[constrainable]
    pub groups: AOption<AVec<AoristRef<UserGroup>>>,
    #[constrainable]
    pub datasets: AOption<AVec<AoristRef<DataSet>>>,
    #[constrainable]
    pub role_bindings: AOption<AVec<AoristRef<RoleBinding>>>,
    #[constrainable]
    pub endpoints: AoristRef<EndpointConfig>,
    #[constrainable]
    pub compliance: AOption<AoristRef<ComplianceConfig>>,
}
pub trait TUniverse {
    fn get_user_unixname_map(&self) -> HashMap<AString, AoristRef<User>>;
    fn get_user_permissions(&self) -> Result<HashMap<AString, HashSet<AString>>, AString>;
}
impl TUniverse for Universe {
    fn get_user_unixname_map(&self) -> HashMap<AString, AoristRef<User>> {
        self.users
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| (x.0.read().get_unixname().clone(), x.clone()))
            .collect()
    }
    fn get_user_permissions(&self) -> Result<HashMap<AString, HashSet<AString>>, AString> {
        let umap = self.get_user_unixname_map();
        let mut map: HashMap<_, HashSet<AString>> = HashMap::new();
        for binding in self.role_bindings.as_ref().unwrap().iter() {
            let name: AString = binding.0.read().get_user_name().clone();
            if !umap.contains_key(&name) {
                return Err(format!("Cannot find user with name {}.", name.as_str())
                    .as_str()
                    .into());
            }
            let user = umap.get(&name).unwrap().clone();
            for perm in binding.0.read().get_role().0.read().get_permissions() {
                map.entry(user.0.read().get_unixname().clone())
                    .or_insert_with(HashSet::new)
                    .insert(perm.clone());
            }
        }
        Ok(map)
    }
}
