use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use crate::role::role::TRole;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct GlobalPermissionsAdmin {}
impl TRole for AoristRef<GlobalPermissionsAdmin> {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
