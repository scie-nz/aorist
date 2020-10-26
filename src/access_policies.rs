#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector { matchLabels: HashMap<String, Vec<String>> },
}

