use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::role::role::TRole;
use aorist_concept::Constrainable;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Derivative, Serialize, Deserialize, Clone, Constrainable)]
#[derivative(PartialEq, Debug, Hash)]
pub struct GlobalPermissionsAdmin {
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq = "ignore", Debug = "ignore", Hash = "ignore")]
    pub constraints: Vec<Arc<RwLock<Constraint>>>,
}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
