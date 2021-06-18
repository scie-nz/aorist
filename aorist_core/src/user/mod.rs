#![allow(non_snake_case)]
use crate::concept::{AoristConcept, ConceptEnum};
use crate::object::TAoristObject;
use crate::role::*;
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    pub unixname: String,
    #[constrainable]
    roles: Option<Vec<Role>>,
}

impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}
