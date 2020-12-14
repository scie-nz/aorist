use crate::role::role::TRole;
use serde::{Deserialize, Serialize};
use aorist_concept::Constrainable;
use crate::concept::AoristConcept;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Eq, Hash,
Constrainable)]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
