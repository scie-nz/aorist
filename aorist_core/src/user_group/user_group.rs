#![allow(non_snake_case)]
use crate::concept::{AoristConcept, ConceptEnum};
use crate::object::TAoristObject;
use crate::user::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[aorist]
pub struct UserGroup {
    name: String,
    members: Vec<String>,
    labels: HashMap<String, String>,
    description: Option<String>,
    users: Vec<User>,
}
pub trait TUserGroup {
    fn get_labels(&self) -> &HashMap<String, String>;
}
impl TUserGroup for UserGroup {
    fn get_labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &String {
        &self.name
    }
}
