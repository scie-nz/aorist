#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::user::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec, TAoristObject};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct UserGroup {
    name: AString,
    members: AVec<AString>,
    labels: BTreeMap<AString, AString>,
    description: Option<AString>,
    users: AVec<AoristRef<User>>,
}
pub trait TUserGroup {
    fn get_labels(&self) -> &BTreeMap<AString, AString>;
}
impl TUserGroup for UserGroup {
    fn get_labels(&self) -> &BTreeMap<AString, AString> {
        &self.labels
    }
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &AString {
        &self.name
    }
}
