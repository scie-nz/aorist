use crate::access_policy::approve_access_selector::*;
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
use aorist_primitives::AUuid;
use aorist_primitives::{AString, AVec};
use aorist_primitives::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[aorist]
pub enum AccessPolicy {
    ApproveAccessSelector(AoristRef<ApproveAccessSelector>),
}
