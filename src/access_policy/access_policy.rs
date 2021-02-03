#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::ApproveAccessSelector;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept2, Constrainable};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept2]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
