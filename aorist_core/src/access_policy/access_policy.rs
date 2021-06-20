#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::*;
use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
