use crate::concept::AoristRef;
use crate::error::AoristError;
use crate::role::*;
use crate::WrappedConcept;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::ConceptEnum;
use aorist_primitives::{AString, AVec, TAoristObject};
use aorist_primitives::{AoristConcept, AoristConceptBase};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct User {
    first_name: AString,
    last_name: AString,
    email: AString,
    phone: AString,
    pub unixname: AString,
    #[constrainable]
    roles: AOption<AVec<AoristRef<Role>>>,
}

impl TAoristObject for User {
    fn get_name(&self) -> &AString {
        &self.unixname
    }
}

pub trait TUser {
    fn to_yaml(&self) -> AString;
    fn get_unixname(&self) -> AString;
    fn set_roles(&mut self, roles: AVec<AoristRef<Role>>) -> Result<(), AoristError>;
    fn get_roles(&self) -> Result<AVec<AoristRef<Role>>, AoristError>;
    fn get_permissions(&self) -> Result<HashSet<AString>, AoristError>;
}
impl TUser for User {
    fn to_yaml(&self) -> AString {
        serde_yaml::to_string(self).unwrap().as_str().into()
    }
    fn get_unixname(&self) -> AString {
        self.unixname.as_str().into()
    }
    fn set_roles(&mut self, roles: AVec<AoristRef<Role>>) -> Result<(), AoristError> {
        if let AOption(ROption::RSome(_)) = self.roles {
            return Err(AoristError::OtherError(AString::from(
                "Tried to set roles more than once.",
            )));
        }
        self.roles = AOption(ROption::RSome(roles));
        Ok(())
    }
    fn get_roles(&self) -> Result<AVec<AoristRef<Role>>, AoristError> {
        match &self.roles {
            AOption(ROption::RSome(x)) => Ok(x.clone()),
            AOption(ROption::RNone) => Err(AoristError::OtherError(AString::from(
                "Tried to get roles for user but set_roles was never called",
            ))),
        }
    }
    fn get_permissions(&self) -> Result<HashSet<AString>, AoristError> {
        let mut perms: HashSet<AString> = HashSet::new();
        for role in self.get_roles()? {
            for perm in role.0.read().get_permissions() {
                perms.insert(perm);
            }
        }
        Ok(perms)
    }
}
