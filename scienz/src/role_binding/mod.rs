use crate::role::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AoristRef, AString, AVec}; 
use aorist_primitives::{
    AoristConcept, AoristConceptBase, ConceptEnum, TAoristObject,
};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct RoleBinding {
    user_name: AString,
    #[constrainable]
    role: AoristRef<Role>,
    name: AString,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &AString {
        &self.name
    }
}
pub trait TRoleBinding {
    fn get_user_name(&self) -> AString;
    fn get_role(&self) -> AoristRef<Role>;
    fn to_yaml(&self) -> AString;
}
impl TRoleBinding for RoleBinding {
    fn get_user_name(&self) -> AString {
        self.user_name.clone()
    }
    fn get_role(&self) -> AoristRef<Role> {
        self.role.clone()
    }
    fn to_yaml(&self) -> AString {
        serde_yaml::to_string(self).unwrap().as_str().into()
    }
}
