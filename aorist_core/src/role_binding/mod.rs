use crate::concept::{AoristRef, WrappedConcept};
use crate::role::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::TAoristObject;
use aorist_primitives::{AoristConcept, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct RoleBinding {
    user_name: String,
    #[constrainable]
    role: AoristRef<Role>,
    name: String,
}
impl TAoristObject for RoleBinding {
    fn get_name(&self) -> &String {
        &self.name
    }
}
pub trait TRoleBinding {
    fn get_user_name(&self) -> &String;
    fn get_role(&self) -> AoristRef<Role>;
    fn to_yaml(&self) -> String;
}
impl TRoleBinding for RoleBinding {
    fn get_user_name(&self) -> &String {
        &self.user_name
    }
    fn get_role(&self) -> AoristRef<Role> {
        self.role.clone()
    }
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).unwrap()
    }
}
