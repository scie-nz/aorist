use crate::concept::{AoristConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod regressor;

pub use self::regressor::*;

#[aorist]
pub enum Model {
    SingleObjectiveRegressor(SingleObjectiveRegressor),
}
