use abi_stable::std_types::ROption;
use aorist_primitives::AOption;

use crate::user::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AUuid;
use aorist_primitives::{AString, AVec, TAoristObject};
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[aorist]
pub struct UserGroup {
    name: AString,
    members: AVec<AString>,
    labels: AVec<AString>,
    description: AOption<AString>,
    users: AVec<AoristRef<User>>,
}
impl TAoristObject for UserGroup {
    fn get_name(&self) -> &AString {
        &self.name
    }
}
