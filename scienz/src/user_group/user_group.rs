use abi_stable::std_types::ROption;
use aorist_util::AOption;

use crate::user::*;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::{AString, AUuid, AVec, AoristRef};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub struct UserGroup {
    name: AString,
    members: AVec<AString>,
    labels: AVec<AString>,
    description: AOption<AString>,
    users: AVec<AoristRef<User>>,
}
