#![allow(non_snake_case)]
use crate::access_policy::approve_access_selector::*;
use crate::concept::{AoristConcept, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[aorist_concept]
pub enum AccessPolicy {
    ApproveAccessSelector(ApproveAccessSelector),
}
