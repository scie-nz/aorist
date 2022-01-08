use crate::concept::{AoristConcept, AoristConceptBase, AoristRef, ConceptEnum};
use abi_stable::std_types::ROption;
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::AOption;
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
