#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::access_policy::approve_access_selector::ApproveAccessSelector;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
