#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, WrappedConcept, ConceptEnum, Concept};
use aorist_concept::{aorist_concept, Constrainable, ConstrainableWithChildren, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod regressor;

pub use self::regressor::*;

#[aorist_concept]
pub enum Model {
    SingleObjectiveRegressor(SingleObjectiveRegressor),
}
