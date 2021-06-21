#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use crate::error::AoristError;
use crate::role::*;
use aorist_concept::{aorist, Constrainable};
use aorist_primitives::TAoristObject;
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[aorist]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    pub unixname: String,
    #[constrainable]
    roles: Option<Vec<AoristRef<Role>>>,
}

impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}

pub trait TUser {
    fn to_yaml(&self) -> String;
    fn get_unixname(&self) -> String;
    fn set_roles(&mut self, roles: Vec<AoristRef<Role>>) -> Result<(), AoristError>;
    fn get_roles(&self) -> Result<Vec<AoristRef<Role>>, AoristError>;
    fn get_permissions(&self) -> Result<HashSet<String>, AoristError>;
}
impl TUser for User {
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
    fn get_unixname(&self) -> String {
        self.unixname.clone()
    }
    fn set_roles(&mut self, roles: Vec<AoristRef<Role>>) -> Result<(), AoristError> {
        if let Some(_) = self.roles {
            return Err(AoristError::OtherError(
                "Tried to set roles more than once.".to_string(),
            ));
        }
        self.roles = Some(roles);
        Ok(())
    }
    fn get_roles(&self) -> Result<Vec<AoristRef<Role>>, AoristError> {
        match &self.roles {
            Some(x) => Ok(x.clone()),
            None => Err(AoristError::OtherError(
                "Tried to get roles for user but set_roles was never called".to_string(),
            )),
        }
    }
    fn get_permissions(&self) -> Result<HashSet<String>, AoristError> {
        let mut perms: HashSet<String> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.0.read().unwrap().get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
    }
}
