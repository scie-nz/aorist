#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::*;
use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
