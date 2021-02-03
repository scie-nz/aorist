#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::ApproveAccessSelector;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::Constrainable;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Constrainable, FromPyObject)]
#[serde(tag = "type", content = "spec")]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
