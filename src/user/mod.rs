#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::error::AoristError;
use crate::object::TAoristObject;
use crate::role::{Role, TRole};
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug, Hash, Eq, Clone)]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
    roles: Option<Vec<Role>>,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore", Hash = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl User {
    pub fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    pub fn get_unixname(&self) -> &String {
        &self.unixname
    }
    pub fn set_roles(&mut self, roles: Vec<Role>) -> Result<(), AoristError> {
        if let Some(_) = self.roles {
            return Err(AoristError::OtherError(
                "Tried to set roles more than once.".to_string(),
            ));
        }
        self.roles = Some(roles);
        Ok(())
    }
    pub fn get_roles(&self) -> Result<Vec<Role>, AoristError> {
        match &self.roles {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get roles for user but set_roles was never called".to_string(),
            )),
        }
    }
    pub fn get_permissions(&self) -> Result<HashSet<String>, AoristError> {
        let mut perms: HashSet<String> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
    }
}
impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}
