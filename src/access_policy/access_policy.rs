#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::ApproveAccessSelector;
use crate::concept::AoristConcept;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}