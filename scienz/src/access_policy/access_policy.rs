use crate::access_policy::approve_access_selector::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_util::AOption;
use aorist_util::AUuid;
use aorist_util::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, ConceptEnum};
use aorist_util::AoristRef;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub enum AccessPolicy {
    #[constrainable]
    ApproveAccessSelector(AoristRef<ApproveAccessSelector>),
}
