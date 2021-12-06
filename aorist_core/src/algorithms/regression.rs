use crate::concept::{AoristConcept, AoristRef, ConceptEnum, WrappedConcept};
use aorist_concept::{aorist, Constrainable};
use aorist_paste::paste;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use aorist_primitives::AString;

#[aorist]
pub struct RandomForestRegressionAlgorithm {}
#[aorist]
pub struct SVMRegressionAlgorithm {}

#[aorist]
pub enum RegressionAlgorithm {
    RandomForestRegressionAlgorithm(AoristRef<RandomForestRegressionAlgorithm>),
    SVMRegressionAlgorithm(AoristRef<SVMRegressionAlgorithm>),
}
