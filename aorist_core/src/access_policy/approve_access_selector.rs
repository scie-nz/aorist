#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use crate::user_group::{TUserGroup, UserGroup};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ApproveAccessSelector {
    matchLabels: AVec<(AString, Vec<AString>)>,
}
pub trait TApproveAccessSelector {
    fn checkGroupIsAllowed(&self, group: &UserGroup) -> bool;
}
impl TApproveAccessSelector for ApproveAccessSelector {
    fn checkGroupIsAllowed(&self, group: &UserGroup) -> bool {
        let my_labels: HashMap<AString, HashSet<AString>> = self
            .matchLabels
            .clone()
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.clone().into_iter().collect::<HashSet<AString>>(),
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
