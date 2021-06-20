use crate::role::*;
use crate::{AoristConcept, ConceptEnum};
use aorist_primitives::TAoristObject;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct RoleBinding {
    user_name: String,
    #[constrainable]
    role: Role,
    name: String,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub trait TRoleBinding {
    fn get_user_name(&self) -> &String;
    fn get_role(&self) -> &Role;
    fn to_yaml(&self) -> String;
}
impl TRoleBinding for RoleBinding {
    fn get_user_name(&self) -> &String {
        &self.user_name
    }
    fn get_role(&self) -> &Role {
        &self.role
    }
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
