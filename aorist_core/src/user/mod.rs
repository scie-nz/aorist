#![allow(non_snake_case)]
use crate::object::TAoristObject;
use crate::role::*;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::concept::{AoristConcept};
use aorist_concept::{aorist, Constrainable};

#[aorist]
pub struct User {
    firstName: String,
    lastName: String,
    email: String,
    phone: String,
    unixname: String,
    #[constrainable]
    roles: Option<Vec<Role>>,
}

impl TAoristObject for User {
    fn get_name(&self) -> &String {
        &self.unixname
    }
}
