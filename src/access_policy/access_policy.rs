#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::ApproveAccessSelector;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
