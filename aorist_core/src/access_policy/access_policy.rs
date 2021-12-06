#![allow(non_snake_case)]
#![allow(unused_parens)]
use crate::access_policy::approve_access_selector::*;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use aorist_primitives::AString;

#[aorist]
pub enum AccessPolicy {
    ApproveAccessSelector(AoristRef<ApproveAccessSelector>),
}
