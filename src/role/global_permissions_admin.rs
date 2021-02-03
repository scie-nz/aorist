use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use crate::role::role::TRole;
use aorist_concept::{aorist_concept2, Constrainable, PythonObject};
use derivative::Derivative;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2(Hash)]
pub struct GlobalPermissionsAdmin {}
impl TRole for GlobalPermissionsAdmin {
    fn get_permissions(&self) -> Vec<String> {
        vec!["gitea/admin".to_string(), "ranger/admin".to_string()]
    }
}
