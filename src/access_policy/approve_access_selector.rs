#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::user_group::UserGroup;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable)]
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
