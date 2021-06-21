#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use derivative::Derivative;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct TimeOrderedTabularSchema {
    pub datumTemplateName: String,
    pub attributes: Vec<String>,
    // non-null time stamp columns used to order records
    // order is always: 1st column, then 2nd, etc.
    pub orderingAttributes: Vec<String>,
}
