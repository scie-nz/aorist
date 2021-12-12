use aorist_primitives::AOption;
use abi_stable::std_types::ROption;
use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

mod regressor;

pub use self::regressor::*;

#[aorist]
pub enum Model {
    SingleObjectiveRegressor(AoristRef<SingleObjectiveRegressor>),
}
