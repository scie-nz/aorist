#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use crate::user_group::UserGroup;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApproveAccessSelector {
    matchLabels: HashMap<String, Vec<String>>,
}
impl ApproveAccessSelector {
    pub fn checkGroupIsAllowed(&self, group: &UserGroup) -> bool {
        let my_labels: HashMap<String, HashSet<&String>> = self
            .matchLabels
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().clone().collect::<HashSet<&String>>()))
            .collect();
        for (k, v) in group.get_labels() {
            if my_labels.contains_key(k) && my_labels[k].contains(v) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}

