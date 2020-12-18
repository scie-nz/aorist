#![allow(non_snake_case)]
use crate::concept::AoristConcept;
use crate::constraint::Constraint;
use crate::user_group::UserGroup;
use aorist_concept::Constrainable;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use uuid::Uuid;
use derivative::Derivative;

#[derive(Derivative, Serialize, Deserialize, Constrainable)]
#[derivative(PartialEq, Debug)]
pub struct ApproveAccessSelector {
    matchLabels: HashMap<String, Vec<String>>,
    uuid: Option<Uuid>,
    #[serde(skip)]
    #[derivative(PartialEq="ignore", Debug="ignore")]
    constraints: Vec<Rc<Constraint>>,
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
