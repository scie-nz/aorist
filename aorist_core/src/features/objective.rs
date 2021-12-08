use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use aorist_primitives::{AString, AVec};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[aorist]
pub struct ContinuousObjective {}

#[aorist]
pub enum ContinuousRegressionObjective {
    ContinuousObjective(AoristRef<ContinuousObjective>),
}

#[aorist]
pub enum RegressionObjective {
    ContinuousRegressionObjective(AoristRef<ContinuousRegressionObjective>),
}
