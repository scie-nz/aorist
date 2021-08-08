#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::user_group::{TUserGroup, UserGroup};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ApproveAccessSelector {
    matchLabels: Vec<(String, Vec<String>)>,
}
pub trait TApproveAccessSelector {
    fn checkGroupIsAllowed(&self, group: &UserGroup) -> bool;
}
impl TApproveAccessSelector for ApproveAccessSelector {
    fn checkGroupIsAllowed(&self, group: &UserGroup) -> bool {
        let my_labels: HashMap<String, HashSet<String>> = self
            .matchLabels
            .clone()
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.clone().into_iter().collect::<HashSet<String>>(),
                )
            })
            .collect();
        for (k, v) in group.get_labels() {
            if my_labels.contains_key(k) && my_labels[k].contains(v) {
                return true;
            }
        }
        return false;
    }
}
