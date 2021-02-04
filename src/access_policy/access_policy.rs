#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::*;
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
