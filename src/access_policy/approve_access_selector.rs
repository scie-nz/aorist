#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use crate::constraint::Constraint;
use crate::user_group::{TUserGroup, UserGroup};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
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
