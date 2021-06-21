use crate::concept::{AoristConcept, AoristRef, WrappedConcept, ConceptEnum};
use aorist_concept::{aorist, Constrainable};
use paste::paste;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod regressor;

pub use self::regressor::*;

#[aorist]
pub enum Model {
    SingleObjectiveRegressor(AoristRef<SingleObjectiveRegressor>),
}
