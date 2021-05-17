#![allow(non_snake_case)]
use crate::concept::{AoristConcept, AoristConceptChildren, Concept};
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod regressor;

pub use self::regressor::{InnerSingleObjectiveRegressor, SingleObjectiveRegressor};

#[aorist_concept]
pub enum Model {
    SingleObjectiveRegressor(SingleObjectiveRegressor),
}
