#![allow(non_snake_case)]
use crate::concept::{AoristConcept, WrappedConcept, ConceptEnum};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use derivative::Derivative;
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[aorist_concept]
pub struct TimeOrderedTabularSchema {
    pub datumTemplateName: String,
    pub attributes: Vec<String>,
    // non-null time stamp columns used to order records
    // order is always: 1st column, then 2nd, etc.
    pub orderingAttributes: Vec<String>,
}
