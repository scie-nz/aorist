use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use aorist_primitives::AString;

mod regressor;

pub use self::regressor::*;

#[aorist]
pub enum Model {
    SingleObjectiveRegressor(AoristRef<SingleObjectiveRegressor>),
}
