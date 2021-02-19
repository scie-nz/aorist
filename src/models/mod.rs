#![allow(non_snake_case)]
use crate::concept::{AoristConcept, Concept};
use crate::constraint::Constraint;
use aorist_concept::{aorist_concept, Constrainable, InnerObject};
use paste::paste;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

mod regressor;

pub use self::regressor::{
    SingleObjectiveRegressor,
    InnerSingleObjectiveRegressor,
};

#[aorist_concept]
pub enum Model {
    SingleObjectiveRegressor(SingleObjectiveRegressor),
}
