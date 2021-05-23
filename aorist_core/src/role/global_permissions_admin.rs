use crate::role::role::TRole;
use crate::concept::AoristConcept;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
